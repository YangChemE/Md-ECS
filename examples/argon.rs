use bevy::prelude::*;
use Md_ECS::{
    simbox::{SimBox},
    atom::*,
    molecular_dynamics::{lj_interaction::*, integration::*},
    setup::*, 
    output::{console::*, file::*},  
    physical_quant_calc::{rdf::{RDF, RDFPlugin}, AnalysisPlugin},
};

use nalgebra::Vector3;
use std::fmt;
use rand_distr::{Distribution, Normal, Uniform};


fn setup_camera(
    mut commands: Commands
) {
    let mut camera = OrthographicCameraBundle::new_3d();
    camera.orthographic_projection.scale = 5e-5;
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

    /* SIMULATION PARAMETERS */
    // atom info 
    let n_atoms: u64 = 5000;

    // integration parameters
    let delta = 2e-12; //2 fs
    let n_steps: u64 = 5000; // 2 ns
    let batch: usize = 50;


    // simulation box parameters
    let len: f64 = 3e-9; // the length of the box, 5 nm
    let box_length = Vector3::new(len, len, len);
    let origin = Vector3::new(0.0, 0.0, 0.0);


    // lennard jones parameters
    let cutoff = 5e-9;

    // output parameters
    let trjname = String::from("./trjs/argon");
    let output_freq = 1;
    let rdf_max = 5.2e-11;
    let rdf_start = 1000;
    let rdf_end = n_steps;



    // Creating an App
    let mut app = App::new();

    // Define our setup plugin,
    // setup plugin includes all the required parameters as fields,
    // all this parameters would be inserted to the world as resources
    // when the plugin in built.
    
    let setup_plugin = SetupPlugin::new(
        n_atoms,
        delta,
        n_steps,
        batch,
        box_length,
        origin,
        cutoff,
        trjname,
        output_freq,
    );


    let rdf_calc_params = RDF::new(
        String::from("Argon"), 
        String::from("Argon"), 
        200, 
        rdf_max,
        rdf_start,
        rdf_end,
        String::from("rdf.csv")
    );

    let rdf_plugin = RDFPlugin::new(rdf_calc_params);

    //app.add_plugins(DefaultPlugins);
    app.add_plugins(MinimalPlugins);
    app.add_plugin(setup_plugin.clone());
    
    app.add_startup_system(create_atoms.label(SetupSystems::CreateAtoms));
    //app.add_startup_system(setup_camera);


    app.add_plugin(LJPlugin);
    app.add_plugin(IntegrationPlugin);
    app.add_plugin(OutputPlugin);
    app.add_plugin(AnalysisPlugin);
    app.add_plugin(rdf_plugin);


    // needs to figure out the purpose of this
    app.add_system(Md_ECS::bevy_bridge::copy_positions);

  


    app.add_system(console_output);

    app.insert_resource(Md_ECS::bevy_bridge::Scale {0: 2e3});
    //app.insert_non_send_resource(LJCutOff{rc: 1e-9});
    println!("done plugins");


    println!("done setup");
    // run the simulation
    for _i in 0..n_steps {
        println!("step {}.", _i);
        app.update();
    }

}


pub fn create_atoms (
    mut commands: Commands,
    n_atoms: Res<AtomNumber>,
    simbox: Res<SimBox>,
) {
    // we use the approximate gas molecule velocity in room temperature as
    // the default value, and we assume the velocity to be isotropic
    let v_dist = Normal::new(0.0, 460.0).unwrap();

    let x_dist = Uniform::new(simbox.origin.x, simbox.origin.x + simbox.dimension.x);
    let y_dist = Uniform::new(simbox.origin.y, simbox.origin.y + simbox.dimension.y);
    let z_dist = Uniform::new(simbox.origin.z, simbox.origin.z + simbox.dimension.z);

    let mut rng = rand::thread_rng();
    let atom_type = 
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
            .insert(Mass {value: 39.948*Md_ECS::constant::AMU})
            .insert(Atom)
            // to be fixed, now the lj parameters are hard coded.
            .insert(AtomType::new(String::from("Argon"), 3.4e-10, 1.654e-21));
    };
}






