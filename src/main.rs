use bevy::prelude::*;

pub struct ECSExample;
 impl Plugin for ECSExample {
     fn build(&self, app: &mut App) {
         app
            .add_startup_system(my_system);
            //.add_system_(my_logic)
            //.add_system(my_other_logic);

     }
 }

 #[derive(Component)]
 struct MycomponentUnit;

 #[derive(Debug, Component)]
 struct MycomponentWithData(String);

 #[derive(Debug, Component)]
 struct MyComponentWithName{
     name: String
 }

struct Target(Entity);

fn my_system (mut commands: Commands) {
    println!("spawned entity {:?}", commands.spawn().insert(MycomponentUnit).id());
    let entity = commands.spawn().insert(MyComponentWithName{
        name: "target".to_string()
    }).id();

    println!("spawned enrity {:?}", commands.spawn().insert(MycomponentWithData("Hello".to_string())).id());
    println!("spawned enrity {:?}", commands.spawn().insert(MycomponentWithData("World".to_string())).id());
    commands.insert_resource(Target(entity));
}

fn main() {
    println!("ECS implementation of MD algorithms.");

}
