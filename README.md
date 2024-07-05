# Blender Theme

[![License](https://img.shields.io/badge/license-GPL--3.0--or--later-blue.svg)](https://github.com/ameknite/blender_theme?tab=readme-ov-file#license)
[![Crates.io](https://img.shields.io/crates/v/blender_theme.svg)](https://crates.io/crates/blender_theme)
[![Minimum Supported Rust Version](https://img.shields.io/badge/MSRV-1.65.0+-red.svg)](./Cargo.toml#L8)
[![CI](https://github.com/ameknite/blender_theme/actions/workflows/ci.yaml/badge.svg)](https://github.com/ameknite/blender_theme/actions/workflows/ci.yaml)

Blender Theme Models.
Facilitate the creation of blender themes.

```toml
[dependencies]
blender_theme = "0.1"
```

## Examples

Using the default blender dark theme.

```rust
use blender_theme::{B3dTheme, Version};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut theme: B3dTheme = Version::V4_2.get_default_theme()?;
    // theme modifications
    theme.save_theme("themes/my_theme.xml")?;

    Ok(())
}
```

Using your own theme.

```rust
use blender_theme::{B3dTheme, Version};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let theme = B3dTheme::from_file("themes/my_theme.xml", &Version::V4_2)?;
    // theme modifications
    theme.save_theme("themes/my_modified_theme.xml")?;

    Ok(())
}
```

## Supported Versions

| blender_theme | blender   |
| ------------- | --------- |
| 0.1           | 3.6 - 4.2 |

## LICENSE

Licensed under the terms of the [GNU General Public License v3.0 or later](LICENSE-GPL-3.0-or-later).

SPDX-License-Identifier: GPL-3.0-or-later
