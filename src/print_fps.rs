use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

#[cfg(debug_assertions)]
pub struct PrintFpsPlugin;

#[cfg(debug_assertions)]
impl Plugin for PrintFpsPlugin {
    fn build(&self, app: &mut App) {
        // Based on https://bevy-cheatbook.github.io/cookbook/print-framerate.html
        app.add_plugin(LogDiagnosticsPlugin::default());
        app.add_plugin(FrameTimeDiagnosticsPlugin::default());
    }
}
