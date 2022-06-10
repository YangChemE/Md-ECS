use bevy::prelude::*;
use crate::atom::*;
use crate::integrator::{CurStep};
use crate::simbox::BoxBound;

pub fn console_output(
    cur_step: Res<CurStep>,
    query: Query<&Atom>,
    boxbound: Res<BoxBound>,
) {
    if cur_step.n % 10 == 0 {
        let atom_number = query.iter().count();

        println!("box position {}, {}, {}.", boxbound.xmax, boxbound.ymax, boxbound.zmax);
    }
}

