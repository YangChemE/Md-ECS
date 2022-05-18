use bevy::{ prelude::*, tasks::ComputeTaskPool };
use rand_distr::{Distribution, Normal};
use Md_ECS::atom::{Atom, Force, Mass, Position, Velocity};
use nalgebra::{Vector3};


fn main() {
    let n_steps: i32 = 10000;
    let n_atoms: i32 = 1000;
    let mut app = App::new();


    let vel_dist = Normal::new(0.0_f64, 0.22_f64).unwrap();
    let pos_dist = Normal::new(0.0_f64, 1.2e-4_f64).unwrap();
    let mut rng = rand::thread_rng();
    // create atoms 
    for _ in 0..n_atoms {
        app.world
            .spawn()
            .insert(
                Position {
                    pos: Vector3::new(
                        pos_dist.sample(&mut rng),
                        pos_dist.sample(&mut rng),
                        pos_dist.sample(&mut rng),
                    ),
                }
            )
            .insert(
                Velocity {
                    vel: Vector3::new(
                    vel_dist.sample(&mut rng),
                    vel_dist.sample(&mut rng),
                    vel_dist.sample(&mut rng),
                    ),
                }
            )
            .insert(Force::default())
            .insert(Mass {value: 2.0})
            .insert(Atom);
    }
    // run the simulation
    for _i in 0..n_steps {
        // calculate forces

        // integration equation of motion (update position and velocity)

        // update time

        // calculate interested quantities (sample average)
    }

}

