use bevy::prelude::*;
use crate::atom::*;
use crate::output::file::{OutputStages, OutputSystems};
use crate::simbox::*;
use crate::molecular_dynamics::integration::{Step, CurStep};
use crate::constant;
use crate::physical_quant_calc::{QuantityCalcStage, QuantityCalcSystems};
use std::fmt::{self, Display};
use std::fs::File;
use std::io::Write;
use std::io::{self, BufWriter};
use std::path::Path;


#[derive(Clone)]
pub struct RDF {
    pub atom_a: String,
    pub atom_b: String,
    pub n_bins: usize,
    pub range: f64,
    pub rdf_cum: (Vec<f64>, Vec<f64>),
    pub filename: String,
}


impl RDF {
    pub fn new(a: String, b:String, bins: usize, rmax: f64, filename: String) -> Self {

        // initialize the r vector
        let mut rs = vec![0.0; bins];
        let rhos = vec![0.0; bins];
        let bin_width = rmax / bins as f64;
        for i in 0..bins {
            rs[i] = bin_width * i as f64;
        }
        Self { 
            atom_a: a, 
            atom_b: b, 
            n_bins: bins, 
            range: rmax, 
            rdf_cum: (rs, rhos),
            filename,
        }
    }
}

fn calc_rdf (
    cur_step: Res<CurStep>,
    tot_step: Res<Step>,
    mut rdf_data: ResMut<RDF>,
    mut simbox: ResMut<SimBox>,
    mut query: Query<(&AtomType, &Position)>,
) {
    let volume = simbox.dimension.x * simbox.dimension.y * simbox.dimension.z;
    let rho_mean = (query.iter().count() as f64) / volume;
    let bin_width = rdf_data.range / rdf_data.n_bins as f64;
    // we need to loop over all pairs, which means this could be more efficient 
    // if we merge this calculation to the force evaluation part, where we also evaluate 
    // the distance between each pair of particles. Might consider add a system resource
    // to serve as a option.

    const K: usize = 2;
    let mut particle_pairs = query.iter_combinations_mut::<K>();

    let mut n_checked_pair = 0.0;

    while let Some([(atom1, pos1), (atom2, pos2)]) 
    = particle_pairs.fetch_next() {
        if (atom1.name == rdf_data.atom_a && atom2.name == rdf_data.atom_b) || (atom1.name == rdf_data.atom_b && atom2.name == rdf_data.atom_a) {
            let mut r = pos1.pos - pos2.pos;

            // treating the pbc 
            r[0] = r[0] - simbox.dimension.x * (r[0]/simbox.dimension.x).round(); 
            r[1] = r[1] - simbox.dimension.y * (r[1]/simbox.dimension.y).round();
            r[2] = r[2] - simbox.dimension.z * (r[2]/simbox.dimension.z).round();

            let distance = r.norm();

            if distance >= rdf_data.range {
                continue;
            }
            let rdf_index =(distance / bin_width).floor() as usize;
            let shell_vol = (4.0 / 3.0) * constant::PI * bin_width.powf(3.0) * ((rdf_index+1).pow(3) - rdf_index.pow(3)) as f64;
            let addition = 2.0 / (shell_vol * rho_mean);// add the counting and normalize it by shell volume and rho_mean at the same time
            rdf_data.rdf_cum.1[rdf_index] += addition; 
            n_checked_pair += 1.0;
        }
    }
    // normalization over all the particle pairs evaluated.
    rdf_data.rdf_cum.1 = rdf_data.rdf_cum.1.clone().into_iter().map(|x| x/(2.0*n_checked_pair)).collect();


    // normalization over all the frames and write to file at the last step.
    if cur_step.n == tot_step.n {
        rdf_data.rdf_cum.1 = rdf_data.rdf_cum.1.clone().into_iter().map(|x| x/(tot_step.n as f64)).collect();

        let filename = rdf_data.filename.clone();
        let path = Path::new(&filename);
        let display = path.display();
        let file = match File::create(&path) {
            Err(why) => panic!("Couldn't open {}: {}", display, why.to_string()),
            Ok(file) => file,
        };

        let mut writer = BufWriter::new(file);
        write_rdf(&mut writer, rdf_data.rdf_cum.clone());
        
    }

}




fn write_rdf<W: Write> (
    writer: &mut W, 
    rdf: (Vec<f64>, Vec<f64>),
) -> Result<(), io::Error> {
    writeln!(writer, "r, g(r)")?;
    for i in 0..rdf.1.len(){
        writeln!(writer, "{}, {}", rdf.0[i], rdf.1[i])?;
    }
    
    Ok(())
}

#[derive(Clone)]
pub struct RDFPlugin {
    params: RDF,
}

impl RDFPlugin {
    pub fn new(rdf:RDF) -> Self {
        Self {
            params: rdf
        }
    }
}
impl Plugin for RDFPlugin {
    fn build(&self, app: &mut App) {
        app.world.insert_resource(self.params.clone());
        app.add_system_to_stage(QuantityCalcStage, calc_rdf.label(QuantityCalcSystems::RdfCalc));
    }
}
