use dirs::home_dir;
use serde::Deserialize;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

// ----------------------------------------------------------------------
// --------------------------------Structs-------------------------------
// ----------------------------------------------------------------------

// Main profile config struct
#[derive(Deserialize, Debug)]
pub struct Profile {
    profile: ProfileConfig,
    packages: PackagesConfig,
    configs: HashMap<String, String>,
    hardware: HashMap<String, String>,
    scripts: HashMap<String, String>,
    exclude: ExcludeConfig,
}

#[derive(Deserialize, Debug)]
struct ProfileConfig {
    name: String,
    version: String,
    author: String,
}

#[derive(Deserialize, Debug)]
struct PackagesConfig {
    aur_helper: String,
    aur: Vec<String>,
    pacman: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct ExcludeConfig {
    paths: Vec<String>,
    patterns: Vec<String>,
}

// ----------------------------------------------------------------------
// --------------------------------Functions-----------------------------
// ---------------------------------------------------------------------

// Check if the config is valid
// Currently only checks if aur helper is empty if aur packages are given
// May be extended in future
fn validate_config(config: &Profile) -> Result<(), Box<dyn std::error::Error>> {
    if config.packages.aur_helper.is_empty() && !config.packages.aur.is_empty() {
        return Err(
            "\x1b[1;31mError\x1b[0m: Aur packages are given but aur helper is empty!".into(),
        );
    } else {
        Ok(())
    }
}

// Check if all paths exist
fn validate_paths(confg: &Profile) -> Result<(), Box<dyn std::error::Error>> {
    let mut non_existent_paths: (Vec<String>, Vec<String>) = (vec![], vec![]);
    for path in &confg.configs {
        if !Path::new(&path.1).exists() {
            non_existent_paths.0.push(path.0.clone());
            non_existent_paths.1.push(path.1.clone());
        }
    }
    for path in &confg.scripts {
        if !Path::new(&path.1).exists() {
            non_existent_paths.0.push(path.0.clone());
            non_existent_paths.1.push(path.1.clone());
        }
    }

    if !non_existent_paths.0.is_empty() {
        // Formatting of string done by ai
        let missing: Vec<String> = non_existent_paths
            .0
            .iter()
            .zip(non_existent_paths.1.iter())
            .map(|(name, path)| format!("  {} -> {}", name, path))
            .collect();
        return Err(format!("The following paths do not exist:\n{}", missing.join("\n")).into());
    }

    Ok(())
}

// Replace ~ with the actual home directory name
fn replace_home_dir(config: &mut Profile) -> &mut Profile {
    let home_directory = home_dir().unwrap();
    for (_, path) in &mut config.configs {
        *path = path.replace("~", home_directory.to_str().unwrap());
    }
    for (_, path) in &mut config.scripts {
        *path = path.replace("~", home_directory.to_str().unwrap());
    }
    for path in &mut config.exclude.paths {
        *path = path.replace("~", home_directory.to_str().unwrap());
    }

    config
}

// Load the config file
pub fn load_config(file: &PathBuf) -> Result<Profile, Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(file)?;
    let mut parsed = toml::from_str(&contents)?;
    validate_config(&parsed)?;
    replace_home_dir(&mut parsed);
    validate_paths(&parsed)?;
    println!("{:#?}", parsed);
    Ok(parsed)
}

// ----------------------------------------------------------------------
// --------------------------------Tests---------------------------------
// ----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_valid_config() {
        let input = r#"
            [profile]
            name = "my-setup"
            version = "0.1.0"
            author = "dhruv"

            [packages]
            aur_helper = "yay"
            aur = ["dms", "7zip"]
            pacman = ["neovim", "git"]

            [configs]
            neovim = "~/.config/nvim"

            [hardware]
            gpu_driver = "nvidia"

            [scripts]
            post_install = "~/.local/scripts/setup.sh"

            [exclude]
            paths = ["~/.config/someapp/session"]
            patterns = ["*.log"]
        "#;

        let result: Profile = toml::from_str(input).unwrap();
        assert_eq!(result.profile.name, "my-setup".to_string());
        assert_eq!(result.packages.pacman, vec!["neovim", "git"]);
    }

    #[test]
    fn test_empty_package_lists() {
        let input = r#"
            [profile]
            name = "minimal"
            version = "0.1.0"
            author = "dhruv"

            [packages]
            aur_helper = "yay"
            aur = []
            pacman = []

            [configs]
            [hardware]
            [scripts]

            [exclude]
            paths = []
            patterns = []
        "#;

        let result: Profile = toml::from_str(input).unwrap();
        assert!(result.packages.aur.is_empty());
        assert!(result.packages.pacman.is_empty());
    }

    #[test]
    fn test_missing_required_field_fails() {
        // author is missing — this should fail to parse
        let input = r#"
            [profile]
            name = "my-setup"
            version = "0.1.0"

            [packages]
            aur_helper = "yay"
            aur = []
            pacman = []

            [configs]
            [hardware]
            [scripts]

            [exclude]
            paths = []
            patterns = []
        "#;

        let result: Result<Profile, _> = toml::from_str(input);
        assert!(result.is_err());
    }
    #[test]
    fn test_malformed_toml_fails() {
        let input = r#"
        this is not valid toml !!!
        [[[
    "#;

        let result: Result<Profile, _> = toml::from_str(input);
        assert!(result.is_err());
    }
}
