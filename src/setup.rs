use bevy::prelude::*;
use crate::{
    atom::AtomNumber,
    molecular_dynamics::{
        integration::{TimeStep, Step, BatchSize, CurStep, Temperature},
        lj_interaction::LJCutOff
    },
    simbox::{SimBox},
    output::file::{TrjName, OutInterval},
};
use nalgebra::{Vector3};




/// build plugin to add resources (the simulation parameters, eg. the integration parameters,
/// the thermostat parameters, the barostat parameters, the output parameters etc.) 

#[derive(Clone)]
pub struct SetupPlugin {
    // system parameteres
    pub temp: Temperature,

    // atoms information
    pub atom_number: AtomNumber,

    // integration parameters
    pub time_step: TimeStep,
    pub number_steps: Step,
    pub batch_size: BatchSize,

    // simulation box parameters
    pub box_size: SimBox,

    // force evaluation parameters
    pub lj_cutoff: LJCutOff,

    // output parameters
    pub cur_step: CurStep,
    pub trj_name: TrjName,
    pub output_interval: OutInterval,
}

impl SetupPlugin {
    pub fn new(
        temp: f64,

        n_atoms: u64,
        delta: f64,
        n_steps: u64,
        batch: usize,

        box_length: Vector3<f64>,
        origin: Vector3<f64>,

        cutoff: f64,

        trjname: String,
        interval: u64,
    ) -> Self {
        let temp = Temperature::new(temp);
        let atom_number = AtomNumber::new(n_atoms);
        let time_step = TimeStep::new(delta);
        let number_steps = Step::new(n_steps);
        let batch_size = BatchSize::new(batch);
        let box_size = SimBox::new(origin, box_length.x, box_length.y, box_length.z);
        let lj_cutoff = LJCutOff::new(cutoff);
        let cur_step = CurStep::init();
        let trj_name = TrjName::new(trjname);
        let output_interval = OutInterval::new(interval);

        Self {
            temp,

            atom_number,
            time_step,
            number_steps,
            batch_size,
            box_size,

            lj_cutoff,
            cur_step,
            trj_name,
            output_interval
        }
    }
}
impl Default for SetupPlugin {
    fn default() -> Self {
        Self { 
            temp: Temperature::default(),
            atom_number: AtomNumber::default(),

            time_step: TimeStep::default(), 
            number_steps: Step::default(), 
            batch_size: default(), 

            box_size: SimBox::default(), 

            lj_cutoff: LJCutOff::default(), 

            cur_step: CurStep::init(),
            trj_name: TrjName::default(), 
            output_interval: OutInterval::default()
        }
    }
}




impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        // add system information
        app.world.insert_resource(self.temp);

        // add atom information
        app.world.insert_resource(self.atom_number);

        // add integration parameters
        app.world.insert_resource(self.batch_size);
        app.world.insert_resource(self.number_steps);
        app.world.insert_resource(self.time_step);

        // add simulation box parameters
        app.world.insert_resource(self.box_size);

        // add lennard jones parameters
        app.world.insert_resource(self.lj_cutoff);

        // add output paramters
        app.world.insert_resource(self.cur_step);
        app.world.insert_resource(self.output_interval);
        app.world.insert_resource(self.trj_name.clone());
    
        //app.add_system_to_stage(CoreStage::Update, deflag_new_atoms.label(InitiateSystems::DeflagNewAtoms));
    }
}




pub mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_setup_plugin() {
        let mut app = App::new();
        let setup_plugin = SetupPlugin::default();
        app.add_plugin(setup_plugin);
        app.update();
        assert!(app.world.contains_resource::<Step>());
    }
}


#[derive(PartialEq, Clone, Hash, Debug, Eq, SystemLabel)]
pub enum SetupSystems {
    CreateAtoms,
}