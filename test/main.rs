use bevy::prelude::*;
#[path="../src/lib.rs"]
use cocoa_bird;


#[derive(Component)]
struct Rotatable;
fn setup(mut commands:Commands,assets:Res<AssetServer>){
    println!("startup system");
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(SpriteBundle{
        texture:assets.load("./demo.png"),
        ..Default::default()
    }).insert(Rotatable);
}

fn rotate(mut query: Query<&mut Transform,With<Rotatable>>,time:Res<Time>){
    for mut transform in query.iter_mut() {
        transform.rotate(Quat::from_rotation_z(time.delta_seconds()));
    }
}
fn main(){
    App::new()
    // .add_plugin(bevy::asset::AssetPlugin)
    // .add_plugin(bevy::core::CorePlugin)
            // .add_plugins(bevy::MinimalPlugins)
            .add_plugin(bevy::log::LogPlugin::default())
            .add_plugin(bevy::core::CorePlugin::default())
            .add_plugin(bevy::time::TimePlugin::default())
            .add_plugin(bevy::transform::TransformPlugin::default())
            .add_plugin(bevy::hierarchy::HierarchyPlugin::default())
            .add_plugin(bevy::diagnostic::DiagnosticsPlugin::default())
            // .add_plugin(bevy::diagnostic::DiagnosticsPlugin::default())
            .add_plugin(bevy::window::WindowPlugin::default())

            .add_plugin(bevy::asset::AssetPlugin::default())
            .add_plugin(bevy::scene::ScenePlugin::default())
            .add_plugin(cocoa_bird::CocoaBirdPlugin)

            // .add_plugin(bevy::winit::WinitPlugin::default())
            // .add_plugin(bevy::input::InputPlugin::default())

            .add_plugin(bevy::render::RenderPlugin::default())
            .add_plugin(bevy::core_pipeline::CorePipelinePlugin::default())
            .add_plugin(bevy::sprite::SpritePlugin::default())
            .add_plugin(bevy::text::TextPlugin::default())
            // .add_plugin(bevy::ui::UiPlugin::default())
            .add_plugin(bevy::pbr::PbrPlugin::default())
            .add_plugin(bevy::gltf::GltfPlugin::default())
            .add_plugin(bevy::animation::AnimationPlugin::default())

            .add_system(rotate)
            .add_startup_system(setup)
            // .add_plugin(bevy::gilrs::GilrsPlugin::default())
            // .add_plugin(bevy::asset::d)
            .run();

}