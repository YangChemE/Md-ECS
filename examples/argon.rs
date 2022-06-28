use bevy::prelude::*;
use Md_ECS::{
    atom::*,
    molecular_dynamics::{lj_interaction::*, integration::*},
    setup::*, 
    output::{console::*, file::*},  
    physical_quant_calc::{rdf::{RDF, RDFPlugin}, AnalysisPlugin},
};

use nalgebra::Vector3;


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
    let n_atoms: u64 = 500;

    // integration parameters
    let delta = 2e-12; //2 fs
    let n_steps: u64 = 1000; // 2 ns
    let batch: usize = 50;


    // simulation box parameters
    let len: f64 = 3e-9; // the length of the box, 5 nm
    let box_length = Vector3::new(len, len, len);
    let origin = Vector3::new(0.0, 0.0, 0.0);


    // lennard jones parameters
    let cutoff = 1.2e-9;

    // output parameters
    let trjname = String::from("./trjs/argon");
    let output_freq = 1;
    let rdf_max = 1.2e-9;
    let rdf_start = n_steps/2;
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
        1000, 
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
    app.add_startup_system(setup_camera);


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







