use bevy::prelude::*;
use crate::atom::*;
use crate::molecular_dynamics::integration::{CurStep, Temperature};
use crate::simbox::SimBox;

pub fn console_output(
    cur_step: Res<CurStep>,
    temp: Res<Temperature>,
    query: Query<&Atom>,
    simbox: Res<SimBox>,
) {
    if cur_step.n % 10 == 0 {
        let atom_number = query.iter().count();

        println!("Step {}, {} atoms, {} K.", cur_step.n, atom_number, 0.0);
    }
}

