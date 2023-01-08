use bevy::prelude::*;
use rusted_shaders::MainPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MainPlugin)
        .run();
}
