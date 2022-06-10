use bevy::prelude::*;
use crate::{
    atom::{AtomNumber, AtomType, Atom},
    integrator::{TimeStep, Step, BatchSize, CurStep},
    simbox::{SimBox, BoxBound},
    lj_interaction::LJCutOff,
    output::file::{TrjName, OutInterval},
};


#[derive(Component, Default)]
pub struct NewlyCreated;

fn deflag_new_atoms(
    mut commands: Commands, 
    query: Query<Entity, With<NewlyCreated>>
) {
    for ent in query.iter() {
        commands.entity(ent).remove::<NewlyCreated>();
    }
}

#[derive(PartialEq, Clone, Hash, Debug, Eq, SystemLabel)]
pub enum InitiateSystems {
    DeflagNewAtoms
}



/// build plugin to add resources (the simulation parameters, eg. the integration parameters,
/// the thermostat parameters, the barostat parameters, the output parameters etc.) 

#[derive(Clone)]
pub struct SetupPlugin {
    // atoms information
    pub atom_number: AtomNumber,

    // integration parameters
    pub time_step: TimeStep,
    pub number_steps: Step,
    pub batch_size: BatchSize,

    // simulation box parameters
    pub box_size: SimBox,
    pub box_bound: BoxBound,

    // force evaluation parameters
    pub lj_cutoff: LJCutOff,

    // output parameters
    pub cur_step: CurStep,
    pub trj_name: TrjName,
    pub output_interval: OutInterval,
}


impl Default for SetupPlugin {
    fn default() -> Self {
        Self { 
            atom_number: AtomNumber::default(),

            time_step: TimeStep::default(), 
            number_steps: Step::default(), 
            batch_size: default(), 

            box_size: SimBox::default(), 
            box_bound: BoxBound::default(), 

            lj_cutoff: LJCutOff::default(), 

            cur_step: CurStep::init(),
            trj_name: TrjName::default(), 
            output_interval: OutInterval::default()
        }
    }
}


impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        // add atom information
        app.world.insert_resource(self.atom_number);

        // add integration parameters
        app.world.insert_resource(self.batch_size);
        app.world.insert_resource(self.number_steps);
        app.world.insert_resource(self.time_step);

        // add simulation box parameters
        app.world.insert_resource(self.box_size);
        app.world.insert_resource(self.box_bound);

        // add lennard jones parameters
        app.world.insert_resource(self.lj_cutoff);

        // add output paramters
        app.world.insert_resource(self.cur_step);
        app.world.insert_resource(self.output_interval);
        app.world.insert_resource(self.trj_name.clone());
    
        //app.add_system_to_stage(CoreStage::Update, deflag_new_atoms.label(InitiateSystems::DeflagNewAtoms));
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}



pub mod tests {
    #[allow(unused_imports)]
    use super::*;

    // testing the 
    #[test]
    fn test_setup_plugin() {
        let mut app = App::new();
        let setup_plugin = SetupPlugin::default();
        app.add_plugin(setup_plugin);
        let test_entity = app.world.spawn().insert(NewlyCreated).id();
        app.update();
        //assert!(!app.world.entity(test_entity).contains::<NewlyCreated>());
        assert!(app.world.contains_resource::<Step>());
    }
}


#[derive(PartialEq, Clone, Hash, Debug, Eq, SystemLabel)]
pub enum SetupSystems {
    CreateAtoms,
}