use bevy::prelude::*;
use nalgebra::{Vector3};

pub struct SimBox {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl SimBox {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z,
        }
    }
}


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