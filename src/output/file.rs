use std::process::Output;
use std::path::Path;
use std::fmt::{self, Display};
use std::fs::File;
use std::io::Write;
use std::io::{self, BufWriter};


/// this file is for defnining the function for outputing
/// the lammps like trajectry file that can be read by ovito.
use crate::atom::*;
use crate::simbox::{SimBox, BoxBound};
use crate::integrator::{Step, Timestep};
use bevy::prelude::*;

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

pub struct OutInterval {
    pub interval: u64,
}

impl OutInterval {
    pub fn new(interval: u64) -> Self {
        Self { interval }
    }
}

impl OutInterval {
    fn default() -> Self {
        Self { interval: 100 }
    }
}

pub struct FrameHeader {
    step: u64,
    atom_number: usize,
    box_bound: BoxBound,
}



pub fn lammps_trj (
    trj_name: Res<TrjName>,
    interval: Res<OutInterval>,
    step: Res<Step>,
    bound: Res<BoxBound>,
    query: Query<(&Position, &Velocity, &Mass)>,
) {
    let atom_number = query.iter().count();
    let box_bound = BoxBound {xmin: bound.xmin, xmax: bound.xmax, ymin: bound.ymin, ymax: bound.ymax, zmin: bound.zmin, zmax: bound.zmax};
    if step.n == 1 {
        let path = format!("{}{}.trj", trj_name.name, interval.interval);
        let path = Path::new(&path);
        let display = path.display();
        let file = match File::create(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why.to_string()),
            Ok(file) => file,
        };
        let mut writer = BufWriter::new(file);
        let header = FrameHeader {step: step.n, atom_number, box_bound};
        write_frame_header(&mut writer, header);

        
    }

    else if step.n % interval.interval == 0 {

    }
}

fn write_frame_header<W: Write> (writer: &mut W, header: FrameHeader) -> Result<(), io::Error> {
    // ITEM: TIMESTEP
    writeln!(writer, "ITEM: TIMESTEP")?;
    // 
    writeln!(writer, "{}", header.step)?;
    //
    writeln!(writer, "ITEM: NUMBER OF ATOMS")?;
    //
    writeln!(writer, "{}", header.atom_number)?;
    //
    writeln!(writer, "ITEM: BOX BOUNDS pp pp pp")?;

    // -X X
    writeln!(writer, "{} {}", header.box_bound.xmin, header.box_bound.xmax)?;
    // -Y Y
    writeln!(writer, "{} {}", header.box_bound.ymin, header.box_bound.ymax)?;
    // -Z Z
    writeln!(writer, "{} {}", header.box_bound.zmin, header.box_bound.zmax)?;

    // ITEM: ATOMS id type x y z vx vy vz speed speed2d temp
    writeln!(
        writer,
        "ITEM: ATOMS id type x y z vx vy vz speed speed2d temp"
    )?;
    Ok(())


}

fn write_atom<W: Write, C: Display> (
    writer: &mut W,
    atom_id: usize,
    data: C,
) -> Result<(), io::Error> {
    writeln!(writer, "{:?} 1 {}", atom_id, data)?;
    Ok(())
}


pub struct OutputPlugin;
impl Plugin for OutputPlugin {
    fn build(&self, app: &mut App) {
        
    }
}