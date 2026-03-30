# Name
archforge - a way to make replicating systems easy

# Description
archforge is a CLI tool that lets users convert their systems into easily replicable objects such as VM images, ISOs, and setup scripts so they can quickly set up a new system.

# Commands

- **scan**  
    Scans your entire filesystem, configs, and packages, then shows recommendations to add to your profile (TOML).

- **build**  
    Builds your profile/TOML file.

  Options:
    - `--output-directory`  
        Path to your output output-directory
    - `--type vm`
        Builds vm a vm image
    - `--type iso`
        Builds arch iso
    - `--type script`
        Builds your setup script

- **tui**
    Shows terminal ui for the archforge

- **profile**
    Performs actions related to profiles

  Options:
    - `list` 
        List your available profiles
    - `apply yourprofilename`
        Apply your profile settings to your current system
