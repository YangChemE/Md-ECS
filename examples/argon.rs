use bevy::{ prelude::*, tasks::ComputeTaskPool };
use Md_ECS::{
    atom::*,
    simbox::{SimBox, BoxBound}, 
    lj_interaction::*, 
    setup::*, 
    output::{console::*, file::*},  
    integrator::*
};

use rand_distr::{Distribution, Normal, Uniform};
use Md_ECS::atom::{Atom, Force, Mass, Position, Velocity, create_atoms};
use nalgebra::{Vector3};


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

fn main() {

    println!("beginning");


    let n_atoms: u64 = 100;

    let delta = 2e-12;
    let n_steps: u64 = 1000;
    let batch: usize = 100;

    let box_len = 1e-8;
    let origin = Vector3::new(0.0, 0.0, 0.0);

    let cutoff = 1.2e-9;
    let output_freq = 10;





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
    let mut setup_plugin = SetupPlugin {
        atom_number: AtomNumber::new(n_atoms),

        time_step: TimeStep::new(delta),
        number_steps: Step::new(n_steps),
        batch_size: BatchSize::new(batch),

        box_size: SimBox::new(box_len, box_len, box_len),
        box_bound: BoxBound::new(origin.x, origin.y, origin.z, SimBox::new(box_len, box_len, box_len)),

        lj_cutoff: LJCutOff::new(cutoff),

        trj_name:  TrjName::new(String::from("argon")),
        output_interval: OutInterval::new(output_freq),

    };

    app.add_plugins(DefaultPlugins);
    app.add_startup_system(create_atoms);
    app.add_startup_system(setup_camera);
    app.add_plugin(setup_plugin.clone());
    app.add_plugin(LJPlugin);
    app.add_plugin(IntegrationPlugin);
    //app.add_plugin(OutputPlugin);

    app.add_system(Md_ECS::bevy_bridge::copy_positions);
    // setup up the simulation, adds all the required parameters
  


    app.add_system(console_output);

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







