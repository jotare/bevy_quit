# bevy_quit

`bevy_quit` is a simple bevy plugin to easily add keybindings to exit bevy games
game.

## Example usage

``` rust
use bevy::prelude::*;

use bevy_quit::QuitPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            QuitPlugin::default() // default will add C-q
                .add_key_binding(KeyCode::Escape)
                .add_key_binding((KeyCode::ControlLeft, KeyCode::W))
                .add_key_binding(vec![
                    KeyCode::ControlLeft,
                    KeyCode::ShiftLeft,
                    KeyCode::AltLeft,
                    KeyCode::C,
                ]),
        )
        .run();
}
```

## Contributing

Contributions are more than welcome. However, to make code more standard,
[`pre-commit`](https://pre-commit.com/) is used. Please, install it and run it
before submitting any code to this repo. Thanks!

To install the `pre-commit` hooks, execute:

``` shell
pre-commit install
```
