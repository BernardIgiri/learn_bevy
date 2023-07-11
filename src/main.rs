mod plugins;

use bevy::prelude::*;
use plugins::PeoplePlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PeoplePlugin))
        .run();
}
