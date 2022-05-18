use crate::atom::*;
use crate::constant;
use bevy::prelude::*;
use bevy::tasks::ComputeTaskPool;


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
pub struct OldForce(Force);

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

}