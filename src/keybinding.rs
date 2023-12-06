use bevy::input::keyboard::KeyCode;

/// Representation of a key binding (from 1 to N simultaneous keys pressed at
/// the same time)
#[derive(Clone)]
pub enum KeyBinding {
    Single(KeyCode),
    Multi(Vec<KeyCode>),
}

/// Single key binding (e.g. ESC)
impl From<KeyCode> for KeyBinding {
    fn from(value: KeyCode) -> Self {
        Self::Single(value)
    }
}

/// Common modifier+character keybindings (e.g. C-q)
impl From<(KeyCode, KeyCode)> for KeyBinding {
    fn from(value: (KeyCode, KeyCode)) -> Self {
        Self::Multi(vec![value.0, value.1])
    }
}

/// Common 2 modifiers and 1 character (e.g. C-M-q)
impl From<(KeyCode, KeyCode, KeyCode)> for KeyBinding {
    fn from(value: (KeyCode, KeyCode, KeyCode)) -> Self {
        Self::Multi(vec![value.0, value.1, value.2])
    }
}

/// Unlimited number of simultaneous keys
impl From<Vec<KeyCode>> for KeyBinding {
    fn from(value: Vec<KeyCode>) -> Self {
        Self::Multi(value)
    }
}
