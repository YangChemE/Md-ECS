use std::path::Path;
use std::fmt::{self, Display};
use std::fs::File;
use std::io::Write;
use std::io::{self, BufWriter};
use nalgebra::Vector3;

/// this file is for defnining the function for outputing
/// the lammps like trajectry file that can be read by ovito.
use crate::atom::*;
use crate::simbox::SimBox;
use crate::molecular_dynamics::integration::{OldForce, CurStep, IntegrationStages, IntegrationSystems};
use bevy::prelude::*;


pub trait Composable: Clone + Send + Sync + 'static {}

#[derive(Debug, Clone)]
pub struct Compose<L, R>(L, R);

impl Composable for Position {}
impl Composable for Velocity {}
impl Composable for Force {}

impl fmt::Display for Compose<Position, Velocity> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Compose(pos, vel) = self;
        write!(f, "{:?} {:?} {:?} ", pos.pos.x, pos.pos.y, pos.pos.z)?;

        let (vx, vy, vz) = (vel.vel.x, vel.vel.y, vel.vel.z);
        write!(f, "{:?} {:?} {:?} ", vx, vy, vz)
    }
}

impl fmt::Display for Compose<Position, Force> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Compose(pos, force) = self;
        write!(f, "{:?} {:?} {:?} ", pos.pos.x, pos.pos.y, pos.pos.z)?;

        let (fx, fy, fz) = (force.force.x, force.force.y, force.force.z);
        write!(f, "{:?} {:?} {:?} ", fx, fy, fz)
    }
}


#[derive(Clone)]
pub struct TrjName {
    pub name: String,
}

impl TrjName {
    pub fn new(filename: String) -> Self {
        Self {
            name: filename
        }
    }
}

impl Default for TrjName {
    fn default() -> Self {
        Self {
            name: String::from("simulation.trj")
        }
    }
}

#[derive(Clone, Copy)]
pub struct OutInterval {
    pub interval: u64,
}

impl OutInterval {
    pub fn new(interval: u64) -> Self {
        Self { interval }
    }
}

impl OutInterval {
    pub fn default() -> Self {
        Self::new(100)
    }
}

pub struct FrameHeader {
    cur_step: u64,
    atom_number: usize,
    origin: Vector3<f64>,
    dimension: Vector3<f64>
}



pub fn lammps_trj (
    trj_name: Res<TrjName>,
    interval: Res<OutInterval>,
    cur_step: Res<CurStep>,
    simbox: Res<SimBox>,
    query: Query<(&Position, &Velocity, &OldForce, &Mass, &AtomID)>,
) {
    let atom_number = query.iter().count();
    let origin = simbox.origin;
    let dimension = simbox.dimension;

    if cur_step.n % interval.interval == 0{
        let filename = format!("{}_{}.trj", trj_name.name, cur_step.n);
        let path = Path::new(&filename);
        let display = path.display();
        let file = match File::create(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why.to_string()),
             Ok(file) => file,
        };
        let mut writer = BufWriter::new(file);
        let header = FrameHeader {cur_step: cur_step.n, atom_number, origin, dimension};
        write_frame_header(&mut writer, header);

        for (pos, vel, old_force, mass, atom_id) in query.iter() {
            write_atom(&mut writer, atom_id.id, Compose(pos.clone(), old_force.0.clone()));
        }
    }
}
    

pub fn first_trj (
    trj_name: Res<TrjName>,
    cur_step: Res<CurStep>,
    simbox: Res<SimBox>,
    query: Query<(&Position, &Velocity, &OldForce, &Mass, &AtomID)>,
) {
    let atom_number = query.iter().count();
    let origin = simbox.origin;
    let dimension = simbox.dimension;    


    if cur_step.n == 0 {
        let filename = format!("{}_{}.trj", trj_name.name, cur_step.n);
        let path = Path::new(&filename);
        let display = path.display();
        let file = match File::create(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why.to_string()),
             Ok(file) => file,
        };
        let mut writer = BufWriter::new(file);
        let header = FrameHeader {cur_step: cur_step.n, atom_number, origin, dimension};
        write_frame_header(&mut writer, header);

        for (pos, vel, old_force, mass, atom_id) in query.iter() {
            write_atom(&mut writer, atom_id.id, Compose(pos.clone(), old_force.0.clone()));
        }
    }
}


fn write_frame_header<W: Write> (writer: &mut W, header: FrameHeader) -> Result<(), io::Error> {
    // ITEM: TIMESTEP
    writeln!(writer, "ITEM: TIMESTEP")?;
    // 
    writeln!(writer, "{}", header.cur_step)?;
    //
    writeln!(writer, "ITEM: NUMBER OF ATOMS")?;
    //
    writeln!(writer, "{}", header.atom_number)?;
    //
    writeln!(writer, "ITEM: BOX BOUNDS pp pp pp")?;

    // -X X
    writeln!(writer, "{} {}", header.origin.x, header.origin.x + header.dimension.x)?;
    // -Y Y
    writeln!(writer, "{} {}", header.origin.y, header.origin.y + header.dimension.y)?;
    // -Z Z
    writeln!(writer, "{} {}", header.origin.z, header.origin.z + header.dimension.z)?;

    // ITEM: ATOMS id type x y z vx vy vz speed speed2d temp
    writeln!(
        writer,
        "ITEM: ATOMS id type x y z vx vy vz"
    )?;
    Ok(())


}

fn write_atom<W: Write, C: Display> (
    writer: &mut W,
    atom_id: u64,
    data: C,
) -> Result<(), io::Error> {
    writeln!(writer, "{:?} 1 {}", atom_id, data)?;
    Ok(())
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum OutputStages {
    FileOutput,
}

#[derive(PartialEq, Clone, Hash, Debug, Eq, SystemLabel)]
pub enum OutputSystems {
    OvitoTrj,
}

pub struct OutputPlugin;
impl Plugin for OutputPlugin {
    fn build(&self, app: &mut App) {

        /// this system is currently running in every update, however my purpose was to output the trajectory of 
        /// the very first frame, which in principle should work when I add the system as startup system, but that didn't 
        /// work somehow, so I put this system here for now until I figure out a better way to do it
        app.add_system_to_stage(IntegrationStages::BeginIntegration, first_trj.before(IntegrationSystems::VelocityVerletIntegratePosition));
        app.add_stage_after(IntegrationStages::EndIntegration, OutputStages::FileOutput, SystemStage::parallel());
        app.add_system_to_stage(OutputStages::FileOutput, lammps_trj.label(OutputSystems::OvitoTrj));
    }
}