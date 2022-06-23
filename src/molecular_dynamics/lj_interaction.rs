use crate::atom::*;
use crate::constant;
use crate::simbox::*;
use crate::molecular_dynamics::integration::*;
use bevy::prelude::*;
use bevy::tasks::ComputeTaskPool;
use nalgebra::Vector3;

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
    mut query: Query<(&mut Force, &mut OldForce, &Position, &AtomType)>,
    //query_j: Query<(&Force, &OldForce, &Position, &LJParams)>
) {
    
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
        
        // adapting the lorentz-berthelot combining rule
        let sigma_12 = (lj_params1.sigma + lj_params2.sigma) / 2.0;
        let epsilon_12 = (lj_params1.epsilon * lj_params2.epsilon).powf(0.5);

        // converting to C12 and C6
        let C12 = 4.0 * epsilon_12 * sigma_12.powf(12.0);
        let C6 = 4.0 * epsilon_12 * sigma_12.powf(6.0);

        if r_square < cut_off.rc.powf(2.0) { // check for cut-off distance
            let lj_ff = 12.0 * C12 / r_square.powf(7.0) - 6.0*C6 / r_square.powf(4.0);
            let lj_force_x = lj_ff * r12[0];
            let lj_force_y = lj_ff * r12[1];
            let lj_force_z = lj_ff * r12[2];
            
            // updating the old force for both particles
            //old_force1.0.force = force1.force;
            //old_force2.0.force = force2.force;
            // updating the force for both particles
            force1.force = force1.force + Vector3::new(lj_force_x, lj_force_y, lj_force_z);
            force2.force = force2.force - Vector3::new(lj_force_x, lj_force_y, lj_force_z);
            //println!("{}, {}, {}", force1.force.x, force1.force.y, force1.force.z);
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