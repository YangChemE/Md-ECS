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


fn integrate_equation_of_motion (
    pool: Res<ComputeTaskPool>,
    batch_size: Res<BatchSize>,
    timestep: ResMut<Timestep>,
    mut step: ResMut<Step>,
    mut query: Query<(&mut Position, &mut Velocity, &mut OldForce, &Force, &Mass)>,
) {
    let dt = timestep.delta;
    query.par_for_each_mut(
        &pool,
        batch_size.0,
        |(mut pos, mut vel, mut old_force, force, mass)| {
            vel.vel += (force.force + old_force.0.force) / (constant::AMU* mass.value) /2.0 * dt;
            pos.pos = pos.pos + vel.vel * dt + force.force / (constant::AMU*mass.value) / 2.0 *dt *dt;
            old_force.0 = *force;
            
        }
    )
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


fn clear_force (
    mut query: Query<&mut Force>,
    pool: Res<ComputeTaskPool>,
    batch_size: Res<BatchSize>,
) {
    query.par_for_each_mut (
        &pool, 
        batch_size.0,
        |mut force| {
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
        app.world.insert_resource(BatchSize::default());
        app.world.insert_resource(Step::default());
        app.world.insert_resource(Timestep::default());

        app.add_stage_before(CoreStage::Update, IntegrationStages::BeginIntegration, SystemStage::parallel());
        app.add_stage_after(CoreStage::Update, IntegrationStages::EndIntegration, SystemStage::parallel());
        app.add_system_to_stage(IntegrationStages::BeginIntegration, 
            velocity_verlet_integrate_position.label(IntegrationSystems::VelocityVerletIntegratePosition));
        app.add_system_to_stage(IntegrationStages::BeginIntegration, 
            clear_force.label(IntegrationSystems::ClearForce).after(IntegrationSystems::VelocityVerletIntegratePosition));
        app.add_system_to_stage(IntegrationStages::BeginIntegration, 
            add_old_force_to_new_atoms.label(IntegrationSystems::AddOldForceToNewAtoms));
        app.add_system_to_stage(IntegrationStages::EndIntegration, 
            velocity_verlet_integrate_velocity.label(IntegrationSystems::VelocityVerletIntegrateVelocity));
    }
}