use bevy::prelude::*;
use crate::atom::*;
use crate::molecular_dynamics::integration::CurStep;
use crate::simbox::SimBox;

pub fn console_output(
    cur_step: Res<CurStep>,
    query: Query<&Atom>,
    simbox: Res<SimBox>,
) {
    if cur_step.n % 10 == 0 {
        let atom_number = query.iter().count();

        println!("Step {}, {} atoms, box origin: {}, {}, {}.", cur_step.n, atom_number, simbox.origin.x, simbox.origin.y, simbox.origin.z);
    }
}

