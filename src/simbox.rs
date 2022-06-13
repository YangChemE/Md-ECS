use bevy::prelude::*;
use nalgebra::{Vector3};

#[derive(Debug, Clone, Copy)]
pub struct SimBox {
    pub origin: Vector3<f64>,
    pub dimension: Vector3<f64>,
}

impl SimBox {
    pub fn new(origin: Vector3<f64>, x_len: f64, y_len: f64, z_len: f64) -> Self {
        Self {
            origin,
            dimension: Vector3::new(x_len, y_len, z_len)
        }
    }
}

impl Default for SimBox {
    fn default() -> Self {
        Self::new(Vector3::new(0.0, 0.0, 0.0), 1e-8, 1e-8, 1e-8)
    }
}

/* 
#[derive(Debug, Clone, Copy)]
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

impl Default for BoxBound {
    fn default() -> Self {
        let default_box_size = SimBox::default();
        Self::new(
            -default_box_size.x/2.0,
            -default_box_size.y/2.0,
            -default_box_size.z/2.0,
            default_box_size,
        )
    }
}
*/