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