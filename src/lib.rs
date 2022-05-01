mod tests;

pub mod drawing;
pub mod util;

use bevy::prelude::*;

pub struct VillPlugin;

impl Plugin for VillPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(drawing::draw_model_sys);
    }
}
