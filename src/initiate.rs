use bevy::prelude::*;


#[derive(Component, Default)]
pub struct NewlyCreated;

fn deflag_new_atoms(
    mut commands: Commands, 
    query: Query<Entity, With<NewlyCreated>>
) {
    for ent in query.iter() {
        commands.entity(ent).remove::<NewlyCreated>();
    }
}

#[derive(PartialEq, Clone, Hash, Debug, Eq, SystemLabel)]
pub enum InitiateSystems {
    DeflagNewAtoms
}

pub struct InitiatePlugin;
impl Plugin for InitiatePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::Update, deflag_new_atoms.label(InitiateSystems::DeflagNewAtoms));
    }
}

pub mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_deflag_new_atom_system() {
        let mut app = App::new();
        app.add_plugin(InitiatePlugin);

        let test_entity = app.world.spawn().insert(NewlyCreated).id();
        app.update();
        assert!(!app.world.entity(test_entity).contains::<NewlyCreated>());
    }
}