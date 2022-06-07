use bevy::{ prelude::*, tasks::ComputeTaskPool };
use Md_ECS::{simbox::{SimBox, BoxBound}, lj_interaction::*, initiate::*, output::{console::*, file::*},  integrator::*};

use rand_distr::{Distribution, Normal};
use Md_ECS::atom::{Atom, Force, Mass, Position, Velocity};
use nalgebra::{Vector3};


fn main() {

    println!("beginning");

    let n_steps: i32 = 1000;
    let n_atoms: i32 = 100;

    let mut app = App::new();

    /* 
    app.insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)));
    app.insert_resource(WindowDescriptor {
        title: "test!".to_string(),
        width: 598.0,
        height: 676.0,
        ..Default::default()
    });
    */
    app.add_plugins(DefaultPlugins);
    app.add_system(Md_ECS::bevy_bridge::copy_positions);
    app.add_plugin(InitiatePlugin);
    app.add_plugin(LJPlugin);
    app.add_plugin(IntegrationPlugin);
    app.add_plugin(OutputPlugin);
    app.add_system(console_output);
    app.add_startup_system(create_atoms);
    app.add_startup_system(setup_camera);
    app.insert_resource(Md_ECS::bevy_bridge::Scale {0: 2e3});
    app.insert_non_send_resource(LJCutOff{rc: 1e-9});
    println!("done plugins");

    app.world.insert_resource(SimBox {x: 1e-8, y: 1e-8, z: 1e-8});

    app.world.insert_resource(BoxBound::new(0.0, 0.0, 0.0, SimBox {x: 1e-8, y: 1e-8, z: 1e-8}));
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


fn create_atoms(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let vel_dist = Normal::new(0.0_f64, 50.0).unwrap();
    let pos_dist = Normal::new(0.0_f64, 1.5e-8).unwrap();
    let mut rng = rand::thread_rng();

    for _ in 0..1000 {
        commands.spawn()
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
            .insert(NewlyCreated)
            .insert_bundle(
                PbrBundle{
                    mesh: meshes.add(Mesh::from(shape::Icosphere {radius: 2e-6, subdivisions: 2})),
                    material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
                    transform: Transform::from_xyz(1.5, 1.5, 1.5),
                    ..default()
                }
            );
    }
}

fn setup_camera(
    mut commands: Commands
) {
    let mut camera = OrthographicCameraBundle::new_3d();
    camera.orthographic_projection.scale = 1e-4;
    camera.transform = Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y);

    commands.spawn_bundle(camera);

    commands.spawn_bundle(
        PointLightBundle {
            transform: Transform::from_xyz(3.0, 8.0, 5.0),
            ..default()
        }
    );
}


