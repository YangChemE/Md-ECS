use bevy::prelude::*;
use nalgebra::{Vector3};

use std::fmt;

#[derive(Clone, Component)]
pub struct Position {
    pub pos: Vector3<f64>,
}

impl Default for Position {
    fn default() -> Self {
        Position {
            pos: Vector3::new(0.0, 0.0, 0.0),
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}, {:?}, {:?})", self.pos[0], self.pos[1], self.pos[2])
    }
}

#[derive(Clone, Copy, Component)]
pub struct Velocity {
    pub vel: Vector3<f64>,
}

impl fmt::Display for Velocity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}, {:?}, {:?})", self.vel[0], self.vel[1], self.vel[2])
    }
}

#[derive(Component)]
pub struct InitialVelocity {
    pub vel: Vector3<f64>,
}

#[derive(Copy, Clone, Component)]
pub struct Force {
    pub force: Vector3<f64>,
}

impl Default for Force {
    fn default() -> Self {
        Force {
            force: Vector3::new(0.0, 0.0, 0.0),
        }
    }
}

#[derive(Clone, Component)]
pub struct Mass {
    /// mass value in atom mass units
    pub value: f64,
}


#[derive(Default, Component)]
pub struct Atom;

#[derive(Component)]
pub struct LJParams {
    pub sigma: f64,
    pub epsilon: f64,
}