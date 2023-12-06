use bevy::app::AppExit;
use bevy::prelude::*;

use crate::KeyBinding;

pub struct QuitPlugin {
    bindings: QuitKeyBindings,
}

#[derive(Clone, Resource)]
struct QuitKeyBindings(Vec<KeyBinding>);

impl QuitPlugin {
    fn new() -> Self {
        Self {
            bindings: QuitKeyBindings(Vec::new()),
        }
    }

    pub fn add_key_binding<K: Into<KeyBinding>>(mut self, keybinding: K) -> Self {
        self.bindings.0.push(keybinding.into());
        self
    }
}

impl Default for QuitPlugin {
    /// Set C-q (ControlLeft, Q) as default quit key binding
    fn default() -> Self {
        Self::new().add_key_binding((KeyCode::ControlLeft, KeyCode::Q))
    }
}

impl Plugin for QuitPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource::<QuitKeyBindings>(self.bindings.clone())
            .add_systems(Update, quit_plugin);
    }
}

fn quit_plugin(
    input: Res<Input<KeyCode>>,
    quit_bindings: Res<QuitKeyBindings>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for binding in quit_bindings.0.iter() {
        let should_exit = match binding {
            KeyBinding::Single(key) => input.pressed(*key),
            KeyBinding::Multi(keys) => keys.iter().all(|key| input.pressed(*key)),
        };
        if should_exit {
            app_exit_events.send(AppExit);
            return;
        }
    }
}
