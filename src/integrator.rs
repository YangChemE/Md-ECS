use crate::atom::*;
use crate::constant;
use crate::lj_interaction;
use crate::simbox::*;
use crate::lj_interaction::*;
use bevy::prelude::*;
use bevy::tasks::ComputeTaskPool;
use nalgebra::Vector3;
use crate::setup::NewlyCreated;

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


fn integrate_equation_of_motion (
    pool: Res<ComputeTaskPool>,
    batch_size: Res<BatchSize>,
    timestep: ResMut<TimeStep>,
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
    timestep: ResMut<TimeStep>,
    mut step: ResMut<Step>,
    boxbound: Res<BoxBound>,
    simbox: Res<SimBox>,
    mut query: Query<(&mut Position, &mut OldForce, &Velocity, &Force, &Mass)>,
) {
    step.n += 1;
    let dt = timestep.delta;

    query.par_for_each_mut(
        &pool,
        batch_size.0,
        |(mut pos, mut old_force, vel, force, mass)|{
            pos.pos = pos.pos + vel.vel * dt + force.force/(constant::AMU*mass.value) / 2.0 * dt * dt;
            pos.pos.x = boxbound.xmin + (pos.pos.x - boxbound.xmin) % simbox.x;
            pos.pos.y = boxbound.ymin + (pos.pos.y - boxbound.ymin) % simbox.y;
            pos.pos.z = boxbound.zmin + (pos.pos.z - boxbound.zmin) % simbox.z;
            old_force.0 = *force;
        }
    );
}

pub const INTEGRATE_VELOCITY_SYSTEM_NAME: &str = "integrate_velocity";

fn velocity_verlet_integrate_velocity (
    pool: Res<ComputeTaskPool>,
    batch_size: Res<BatchSize>,
    timestep: Res<TimeStep>,
    mut query: Query<(&mut Velocity, &Force, &OldForce, &Mass)>,
) {
    let dt = timestep.delta;
    //println!("integration running!");
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
        app.add_system_to_stage(IntegrationStages::BeginIntegration, 
            add_old_force_to_new_atoms.label(IntegrationSystems::AddOldForceToNewAtoms));
    }
}

pub mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_add_old_force_system() {
        let mut app = App::new();
        app.add_plugin(IntegrationPlugin);

        let test_entity = app.world.spawn().insert(NewlyCreated).id();
        app.update();
        assert!(
            app.world.entity(test_entity).contains::<OldForce>(),
            "OldForce component not added to test entity."
        );
    }
}