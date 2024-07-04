// SPDX-License-Identifier: GPL-3.0-or-later

//! Blender Theme

use blender_theme::Version;

#[test]
fn create_themes() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let versions = [Version::V3_6, Version::V4_0, Version::V4_1, Version::V4_2];
    let mut original_themes = Vec::with_capacity(versions.len());

    for version in &versions {
        original_themes.push(version.get_theme()?);
    }

    for theme in &original_themes {
        theme.create_theme("themes/created", &theme.version.to_string())?;
    }

    Ok(())
}
