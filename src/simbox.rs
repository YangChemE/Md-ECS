use bevy::prelude::*;
use nalgebra::{Vector3};

pub struct SimBox {
    pub x: f64,
    pub y: f64,
    pub z: f64
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