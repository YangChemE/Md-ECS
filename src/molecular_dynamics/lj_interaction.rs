use crate::atom::*;
use crate::constant;
use crate::simbox::*;
use crate::molecular_dynamics::integration::*;
use crate::physical_quant_calc::rdf::*;
use bevy::prelude::*;
use bevy::tasks::ComputeTaskPool;
use nalgebra::Vector3;
use std::fs::File;
use std::io::Write;
use std::io::{self, BufWriter};
use std::path::Path;

#[derive(Clone, Copy)]
pub struct LJCutOff {
    pub rc: f64,
}

impl LJCutOff {
    pub fn new(rc: f64) -> Self{
        Self { rc }
    }
}

impl Default for LJCutOff {
    fn default() -> Self {
        Self::new(1.2e-9)
    }
}


pub fn calc_lj_force (
    pool: Res<ComputeTaskPool>,
    batch_size: Res<BatchSize>,
    timestep: ResMut<TimeStep>,
    box_size: ResMut<SimBox>,
    cut_off: ResMut<LJCutOff>,
    mut rdf_data: ResMut<RDF>,
    cur_step: Res<CurStep>,
    tot_step: Res<Step>,
    mut query: Query<(&mut Force, &mut OldForce, &Position, &AtomType)>,
    //query_j: Query<(&Force, &OldForce, &Position, &LJParams)>
) {
    // box volume
    let box_v = box_size.dimension.x * box_size.dimension.y * box_size.dimension.z;
    // 
    let np = query.iter().count() as f64;
    // rho_mean 
    let rho_mean = (query.iter().count() as f64) / box_v;
    // bin width
    let bin_width = rdf_data.range / rdf_data.n_bins as f64;
    // for recording checked pair
    //let mut n_checked_pair = 0.0;
    // initiate an array for recording the rdf for this single frame
    let mut cur_rhos = vec![0.0; rdf_data.n_bins];

    const K: usize = 2;
    let mut particle_combos = query.iter_combinations_mut::<K>();

    while let Some([(mut force1, mut old_force1, pos1, atom1), (mut force2, mut old_force2, pos2, atom2)]) 
    = particle_combos.fetch_next() {
        //println!("force calculation!");
        let lj_params1 = atom1.lj_params;
        let lj_params2 = atom2.lj_params;
        // here we have a pair of atoms in the system labeled as 1 and 2 for calculating the interaction between them.
        // since the iter_combo methods returns a combinations of targeted entity withou repeatation 
        // we can calculate the force asserted on each atom of the combo and update the said force.
        // The LJ potential is in form of Vij = 4*epsilon * [ (sigma/r)^12 - (sigma/r)^6 ] which gives force in the form of 
        // fi = dV/dr * rij/r = 
        let mut r12 = pos1.pos - pos2.pos; // use the second atom as reference

        // the pbc treatment
        r12[0] = r12[0] - box_size.dimension.x * (r12[0]/box_size.dimension.x).round(); 
        r12[1] = r12[1] - box_size.dimension.y * (r12[1]/box_size.dimension.y).round();
        r12[2] = r12[2] - box_size.dimension.z * (r12[2]/box_size.dimension.z).round();

        let r_square = r12.norm_squared();
        let r = r12.norm();

        if cur_step.n >= rdf_data.start-1 && cur_step.n <= rdf_data.end-1 && r <= rdf_data.range {
            let rdf_ndx = (r / bin_width).floor() as usize;
            cur_rhos[rdf_ndx] += 2.0;
        }


        // adapting the lorentz-berthelot combining rule
        let sigma_12 = (lj_params1.sigma + lj_params2.sigma) / 2.0;
        let epsilon_12 = (lj_params1.epsilon * lj_params2.epsilon).powf(0.5);

        // converting to C12 and C6
        let C12 = 4.0 * epsilon_12 * sigma_12.powf(12.0);
        let C6 = 4.0 * epsilon_12 * sigma_12.powf(6.0);

        if r_square < cut_off.rc.powf(2.0) { // check for cut-off distance
            let lj_ff = 12.0 * C12 / r_square.powf(7.0) - 6.0 * C6 / r_square.powf(4.0);
            let lj_force_x = lj_ff * r12[0];
            let lj_force_y = lj_ff * r12[1];
            let lj_force_z = lj_ff * r12[2];
            
            // updating the force for both particles
            force1.force = force1.force + Vector3::new(lj_force_x, lj_force_y, lj_force_z);
            force2.force = force2.force - Vector3::new(lj_force_x, lj_force_y, lj_force_z);
        }
    }
    
    // normalize the radial rho by the mean rho of the system and the number of atoms calculated for

    if cur_step.n >= rdf_data.start-1 && cur_step.n <= rdf_data.end-1 {
        for i in 0..rdf_data.rdf_cum.1.len() {
            let r_outer = bin_width * (i+1) as f64;
            let r_inner = bin_width * i as f64;
            let shell_vol = (4.0/3.0)*constant::PI*(r_outer.powf(3.0) - r_inner.powf(3.0));
    
            rdf_data.rdf_cum.1[i] += cur_rhos[i] / (np*rho_mean*shell_vol);
        }
    
        if cur_step.n == rdf_data.end-1 {

            // normalize over the used frames
            rdf_data.rdf_cum.1 = rdf_data.rdf_cum.1.clone().into_iter().map(|x| x/(rdf_data.end - rdf_data.start) as f64).collect();
    
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
    
}



#[derive(PartialEq, Clone, Hash, Debug, Eq, StageLabel)]
pub enum ForceStages {
    LJStage,
}

#[derive(PartialEq, Clone, Hash, Debug, Eq, SystemLabel)]
pub enum ForceSystems {
    LJSystem,
}

pub struct LJPlugin;
impl Plugin for LJPlugin {
    fn build(&self, app: &mut App) {
        app.add_stage_before(CoreStage::Update, ForceStages::LJStage, SystemStage::parallel());
        app.add_system_to_stage(ForceStages::LJStage, calc_lj_force.label(ForceSystems::LJSystem));
    }
}
/* 
pub mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_calc_lj_force() {
        let mut app = App::new();
        app.add_plugin()
    }
}

*/