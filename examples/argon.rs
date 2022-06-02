use bevy::{ prelude::*, tasks::ComputeTaskPool };
use Md_ECS::{simbox::SimBox, lj_interaction::*, initiate::*, output::{console::*, file::*},  integrator::*};

use rand_distr::{Distribution, Normal};
use Md_ECS::atom::{Atom, Force, Mass, Position, Velocity};
use nalgebra::{Vector3};


fn main() {

    println!("beginning");

    let n_steps: i32 = 10000;
    let n_atoms: i32 = 1000;

    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugin(InitiatePlugin);
    app.add_plugin(LJPlugin);
    app.add_plugin(IntegrationPlugin);
    app.add_plugin(OutputPlugin);
    app.add_system(console_output);

    println!("done plugins");

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
            .insert(Atom)
            .insert(NewlyCreated);
    }

    app.world.insert_resource(SimBox {
        x: 0.1,
        y: 0.1,
        z: 0.1
    });
    println!("done setup");
    // run the simulation
    for _i in 0..n_steps {
        println!("step {}.", _i);
        app.update();
        // calculate forces

        // integration equation of motion (update position and velocity)

        // update time

        // calculate interested quantities (sample average)
    }

}

