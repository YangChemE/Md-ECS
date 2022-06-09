use bevy::prelude::*;
use nalgebra::{Vector3};
use crate::{simbox::{SimBox, BoxBound}, integrator::OldForce};
use std::fmt;
use rand_distr::{Distribution, Normal, Uniform};

#[derive(Clone, Component)]
pub struct AtomID {
    pub id: u64,
}

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

#[derive(Clone, Component)]
pub struct AtomType {
    /// the name of the atom
    pub name: String,
}


#[derive(Default, Component)]
pub struct Atom;

#[derive(Clone, Copy)]
pub struct AtomNumber {
    pub n_atoms: u64,
}

impl AtomNumber {
    pub fn new(n: u64) -> Self {
        Self { n_atoms: n }
    }
}

impl Default for AtomNumber {
    fn default() -> Self {
        Self::new(1000)
    }
}

#[derive(Component)]
pub struct LJParams {
    pub sigma: f64,
    pub epsilon: f64,
}

pub fn create_atoms (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    n_atoms: Res<AtomNumber>,
    box_bound: Res<BoxBound>,
) {
    // we use the approximate gas molecule velocity in room temperature as
    // the default value, and we assume the velocity to be isotropic
    let v_dist = Normal::new(460.0, 50.0).unwrap();

    let x_dist = Uniform::new(box_bound.xmin, box_bound.xmax);
    let y_dist = Uniform::new(box_bound.ymin, box_bound.ymax);
    let z_dist = Uniform::new(box_bound.zmin, box_bound.zmax);

    let mut rng = rand::thread_rng();

    for i in 0..n_atoms.n_atoms {
        commands.spawn()
            .insert(
                Position {
                    pos: Vector3::new (
                        x_dist.sample(&mut rng),
                        y_dist.sample(&mut rng),
                        z_dist.sample(&mut rng),
                    )
                }
            )
            .insert(AtomID {id: i+1})
            .insert(
                Velocity {
                    vel: Vector3::new(
                        v_dist.sample(&mut rng),
                        v_dist.sample(&mut rng),
                        v_dist.sample(&mut rng),
                    )
                }
            )
            .insert(Force::default())
            .insert(OldForce(Force::default()))
            .insert(Mass {value: 39.948*crate::constant::AMU})
            .insert(Atom)
            // for rendering purpose
            .insert_bundle(
                PbrBundle{
                    mesh: meshes.add(Mesh::from(shape::Icosphere {radius: 1e-6, subdivisions: 2})),
                    material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
                    transform: Transform::from_xyz(1.5, 1.5, 1.5),
                    ..default()
                }
            );
    }
}

//fn create_atoms_system () -> Fn()