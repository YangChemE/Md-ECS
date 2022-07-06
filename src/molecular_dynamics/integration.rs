use crate::atom::*;
use crate::constant;
use crate::simbox::*;
use bevy::prelude::*;
use bevy::tasks::ComputeTaskPool;
use nalgebra::Vector3;



#[derive(Clone, Copy)]
pub struct CurStep {
    pub n: u64,
}

impl CurStep {
    pub fn init() -> Self {
        Self {n: 0}
    }
}

#[derive(Clone, Copy, Component)]
pub struct Temperature {
    pub sumv: Vector3<f64>,
    pub sumv2: f64,
    pub t_cur: f64,
    pub t_ref: f64,
}

impl Temperature {
    pub fn new(t_ref: f64) -> Self {
        Self {sumv: Vector3::new(0.0, 0.0, 0.0), sumv2: 0.0, t_cur: t_ref, t_ref}
    }
}

impl Default for Temperature {
    fn default() -> Self {
        Self { sumv: Vector3::new(0.0, 0.0, 0.0), sumv2: 0.0, t_cur: 298.15, t_ref: 298.15}
    }
}


#[derive(Clone, Copy)]
pub struct Step {
    pub n: u64,
}

impl Step {
    pub fn new(n_step: u64) -> Self {
        Self { n: n_step }
    }
}

impl Default for Step {
    fn default() -> Self {
        Step { n: 1000 }
    }
}

#[derive(Clone, Copy)]
pub struct TimeStep {
    pub delta: f64,
}

impl TimeStep {
    pub fn new(dt: f64) -> Self {
        Self {
            delta: dt
        }
    }
}

impl Default for TimeStep {
    fn default() -> Self {
        TimeStep { delta: 2e-12} // 2 femtoseconds
    }
}

#[derive(Component, Default)]
pub struct OldForce(pub Force);

pub const INTEGRATE_POSITION_SYSTEM_NAME: &str = "integrate_position";

#[derive(Clone, Copy)]
pub struct BatchSize(pub usize);

impl BatchSize {
    pub fn new(batch: usize) -> Self{
        Self(batch)
    }
}
impl Default for BatchSize {
    fn default() -> Self {
        BatchSize::new(1000)
    }
}



fn velocity_verlet_integrate_position (
    pool: Res<ComputeTaskPool>,
    batch_size: Res<BatchSize>,
    mut temp: ResMut<Temperature>,
    timestep: ResMut<TimeStep>,
    simbox: Res<SimBox>,
    mut query: Query<(&mut Position, &Velocity, &Force, &Mass)>,
) {
    temp.sumv = Vector3::new(0.0, 0.0, 0.0);
    temp.sumv2 = 0.0;
    let dt = timestep.delta;
    query.par_for_each_mut(
        &pool,
        batch_size.0,
        |(mut pos, vel, force, mass)|{
            pos.pos = pos.pos + vel.vel * dt + (force.force/(constant::AMU*mass.value) / 2.0) * dt * dt;

            // to deal with the pbc
            pos.pos.x = pbc(pos.pos.x, simbox.origin.x, simbox.dimension.x);
            pos.pos.y = pbc(pos.pos.y, simbox.origin.y, simbox.dimension.y);
            pos.pos.z = pbc(pos.pos.z, simbox.origin.z, simbox.dimension.z);

        }
    );
}



fn pbc (coord: f64, min: f64, range:f64) -> f64 {
    let max = min + range;
    let _coord = coord - min;
    let mut new_coord = 0.0;
    if _coord < 0.0 {

        new_coord = max - (-_coord % range);
    }
    else {
        new_coord = min +  (_coord % range);
    }

    new_coord
}

pub const INTEGRATE_VELOCITY_SYSTEM_NAME: &str = "integrate_velocity";

fn velocity_verlet_integrate_velocity (
    pool: Res<ComputeTaskPool>,
    batch_size: Res<BatchSize>,
    timestep: Res<TimeStep>,
    mut temp: ResMut<Temperature>,
    mut cur_step: ResMut<CurStep>,
    mut query: Query<(&mut Velocity, &Force, &OldForce, &Mass)>,
) {
    let dt = timestep.delta;
    let atom_number = query.iter().count() as f64;
    let mut sumv = Vector3::new(0.0, 0.0, 0.0);
    let mut sumv2 = 0.0;

    // updating the velocities
    query.par_for_each_mut (
        &pool,
        batch_size.0,
        |(mut vel, force, old_force, mass)| {
            vel.vel += (force.force + old_force.0.force) / (constant::AMU * mass.value) / 2.0 * dt;
        }    
    );

    // calculating the COM vel
    for (vel, _, _, _) in query.iter() {
        sumv += vel.vel;  
    }
    let v_com = sumv/atom_number;
    //println!("v com , {}", v_com);
    for (mut vel, _, _, _) in query.iter_mut() {
        // substract the COM vel
        vel.vel = vel.vel - v_com;
        sumv2 += vel.vel.norm_squared();       
    }

    temp.t_cur = (constant::AMU * 39.948 * sumv2) / atom_number / (3.0 * constant::BOLTZCONST);
    println!("current T: {}.", temp.t_cur);
    cur_step.n += 1;
}



fn clear_force (
    mut query: Query<(&mut Force, &mut OldForce)>,
    pool: Res<ComputeTaskPool>,
    batch_size: Res<BatchSize>,
) {
    query.par_for_each_mut (
        &pool, 
        batch_size.0,
        |(mut force, mut old_force)| {
            old_force.0.force = force.force;
            force.force = Vector3::new(0.0, 0.0, 0.0);
        }
    )
}


#[derive(PartialEq, Clone, Hash, Debug, Eq, SystemLabel)]
pub enum IntegrationSystems {
    VelocityVerletIntegratePosition,
    VelocityVerletIntegrateVelocity,
    AddOldForceToNewAtoms,
    ClearForce,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum IntegrationStages {
    BeginIntegration,
    EndIntegration,
}

pub struct IntegrationPlugin;
impl Plugin for IntegrationPlugin {
    fn build(&self, app: &mut App) {
        // we add two stages after core stage for the position update and velocity update respectively
        app.add_stage_after(CoreStage::Update, IntegrationStages::BeginIntegration, SystemStage::parallel());
        app.add_stage_after(IntegrationStages::BeginIntegration, IntegrationStages::EndIntegration, SystemStage::parallel());
        

        // we add the position updating system to the begin integration stage
        app.add_system_to_stage(IntegrationStages::BeginIntegration, 
            velocity_verlet_integrate_position.label(IntegrationSystems::VelocityVerletIntegratePosition));

        //  we add the velocity updating system to the end integration stage
        app.add_system_to_stage(IntegrationStages::EndIntegration,
            velocity_verlet_integrate_velocity.label(IntegrationSystems::VelocityVerletIntegrateVelocity));

        // then we store the current force to old force then clear the current force after velocity updating
        app.add_system_to_stage(IntegrationStages::EndIntegration, 
            clear_force.label(IntegrationSystems::ClearForce).after(IntegrationSystems::VelocityVerletIntegrateVelocity));

        // This is only useful when we need to add atoms during the simulation,
        // which we are not implementing yet.
        // app.add_system_to_stage(IntegrationStages::BeginIntegration, 
        // add_old_force_to_new_atoms.label(IntegrationSystems::AddOldForceToNewAtoms));
    }
}

