/// this file is for defnining the function for outputing
/// the lammps like trajectry file that can be read by ovito.
use crate::atom::*;
use crate::simbox::*;
use crate::integrator::{Step, Timestep};
use bevy::prelude::*;

pub fn lammps_trj (
    step: Res<Step>,
    query: Query<&Atom>,
) {
    if step.n == 1 {

    }
    else if step.n % 100 == 0 {

    }
}