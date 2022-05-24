use crate::atom::*;
use crate::constant;
use crate::simbox::*;
use crate::lj_interaction::*;
use bevy::prelude::*;
use bevy::tasks::ComputeTaskPool;
use nalgebra::Vector3;
use crate::initiate::NewlyCreated;

pub struct Step {
    pub n: u64,
}


impl Default for Step {
    fn default() -> Self {
        Step { n: 0 }
    }
}

pub struct Timestep {
    pub delta: f64,
}

impl Default for Timestep {
    fn default() -> Self {
        Timestep { delta: 2e-12} // 2 femtoseconds
    }
}

#[derive(Component, Default)]
pub struct OldForce(pub Force);

pub const INTEGRATE_POSITION_SYSTEM_NAME: &str = "integrate_position";

pub struct BatchSize(pub usize);
impl Default for BatchSize {
    fn default() -> Self {
        BatchSize(1024)
    }
}

fn velocity_verlet_integrate_position (
    pool: Res<ComputeTaskPool>,
    batch_size: Res<BatchSize>,
    timestep: ResMut<Timestep>,
    mut step: ResMut<Step>,
    mut query: Query<(&mut Position, &mut OldForce, &Velocity, &Force, &Mass)>,
) {
    step.n += 1;
    let dt = timestep.delta;

    query.par_for_each_mut(
        &pool,
        batch_size.0,
        |(mut pos, mut old_force, vel, force, mass)|{
            pos.pos = pos.pos + vel.vel * dt + force.force/(constant::AMU*mass.value) / 2.0 * dt * dt;
            old_force.0 = *force;
        }
    );
}

pub const INTEGRATE_VELOCITY_SYSTEM_NAME: &str = "integrate_velocity";

fn velocity_verlet_integrate_velocity (
    pool: Res<ComputeTaskPool>,
    batch_size: Res<BatchSize>,
    timestep: Res<Timestep>,
    mut query: Query<(&mut Velocity, &Force, &OldForce, &Mass)>,
) {
    let dt = timestep.delta;

    query.par_for_each_mut (
        &pool,
        batch_size.0,
        |(mut vel, force, old_force, mass)| {
            vel.vel += (force.force + old_force.0.force) / (constant::AMU * mass.value) / 2.0 * dt;
        }    
    );
}

fn add_old_force_to_new_atoms(
    mut commands: Commands,
    query: Query<Entity, (With<NewlyCreated>, Without<OldForce>)>
) {
    for ent in query.iter() {
        commands.entity(ent).insert(OldForce::default());
    }
}
