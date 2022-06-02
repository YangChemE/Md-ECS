use bevy::prelude::*;
use crate::atom::*;
use crate::integrator::{Step, Timestep};


pub fn console_output(
    step: Res<Step>,
    query: Query<&Atom>,
) {
    if step.n % 100 == 0 {
        let atom_number = query.iter().count();
        println!("Step {}: simulating {} atoms.", step.n, atom_number)
    }
}

pub struct OutputPlugin;
impl Plugin for OutputPlugin {
    fn build(&self, app: &mut App) {
        
    }
}