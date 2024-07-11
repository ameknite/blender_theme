// SPDX-License-Identifier: GPL-3.0-or-later

//! Blender Theme

use std::path::PathBuf;

use blender_theme::B3dTheme;

#[test]
fn create_themes() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let versions = B3dTheme::all_versions();
    let mut original_themes = Vec::with_capacity(versions.len());

    for version in &versions {
        original_themes.push(version.get_default_theme()?);
    }

    for theme in &original_themes {
        let save_path = PathBuf::from("themes/created");
        let save_path = save_path.join(theme.version.file_name());
        theme.save_theme(&save_path)?;
    }

    Ok(())
}
