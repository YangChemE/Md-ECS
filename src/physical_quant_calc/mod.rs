use bevy::prelude::*;
pub mod rdf;
use crate::output::file::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub struct QuantityCalcStage;


#[derive(PartialEq, Clone, Hash, Debug, Eq, SystemLabel)]
pub enum QuantityCalcSystems{
    RdfCalc,
}

pub struct AnalysisPlugin;

impl Plugin for AnalysisPlugin {
    fn build(&self, app: &mut App) {
        app.add_stage_after(OutputStages::FileOutput, QuantityCalcStage, SystemStage::parallel());
    }
}

#[derive(Clone, Copy)]
pub enum RDFOption {
    On,
    Off,
}