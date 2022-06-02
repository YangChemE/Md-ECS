/// this file is for defnining the function for outputing
/// the lammps like trajectry file that can be read by ovito.
use crate::simbox::SimBox;
use crate::atom::*;
use crate::integrator::{Step, Timestep};
use bevy::prelude::*;

#[derive(Debug)]
pub struct BoxBound {
    pub xmin: f64,
    pub xmax: f64,
    pub ymin: f64,
    pub ymax: f64,
    pub zmin: f64,
    pub zmax: f64,
}

impl BoxBound {
    pub fn new(xmin: f64, ymin: f64, zmin: f64, simbox: SimBox) -> Self {
        let xmax = xmin + simbox.x;
        let ymax = ymin + simbox.y;
        let zmax = zmin + simbox.z;
        Self {
            xmin,
            xmax,
            ymin,
            ymax,
            zmin,
            zmax,
        }
    }
}

pub fn lammps_output (
    step: Res<Step>,
    query: Query<&Atom>,
) {
    
}