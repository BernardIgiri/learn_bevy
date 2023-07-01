mod plugins;

use bevy::prelude::*;
use plugins::PeoplePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PeoplePlugin)
        .run();
}
