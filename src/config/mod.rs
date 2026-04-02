use serde::Deserialize;
use std::{
    collections::{ HashMap},
    path::PathBuf,
};

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

pub fn load_config(file: &PathBuf) -> Profile {
    let  contents = std::fs::read_to_string(file).expect("Path not found");
    let parsed = toml::from_str(&contents).unwrap();
    println!("{:#?}", parsed);
    parsed
}


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
}
