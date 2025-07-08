use std::{
    env,
    fs::{self},
    io::Write,
};

use clap::Parser;
use console::style;
use inquire::{Select, Text};

#[derive(Debug, Parser)]
struct Args {
    profile: Option<String>,
    #[clap(long, default_value = "false")]
    success_to_switch: bool,
}

#[derive(Debug, Clone)]
struct Profile {
    name: String,
    path: String,
}

fn get_config_dir() -> String {
    let home_dir = env::var_os("HOME").expect("Failed to read $HOME env");
    let dir_path = env::var("CCSWITCH_PROFILE_DIR").unwrap_or(format!(
        "{}/{}",
        home_dir.into_string().unwrap(),
        ".config/ccswitch"
    ));

    dir_path
}

fn get_profiles() -> Vec<Profile> {
    let dir_path = get_config_dir();

    if !fs::exists(&dir_path).expect("Failed to check directory existence") {
        fs::create_dir_all(&dir_path).expect("Failed to create directory");
    }

    let dir = fs::read_dir(&dir_path).expect("Failed to read directory");
    let profiles = dir
        .filter_map(|e| {
            let entry = e.expect("Failed to read directory entry");

            if !entry.file_type().expect("Failed to get file type").is_dir() {
                return None;
            }

            let path = entry.path();
            let name = path.file_name().unwrap().to_str().unwrap().to_string();
            Some(Profile {
                name,
                path: path.to_string_lossy().into_owned(),
            })
        })
        .collect();
    profiles
}

fn create_profile(name: String) -> Profile {
    let dir_path = get_config_dir();
    let profile_path = format!("{}/{}", dir_path, name);
    fs::create_dir_all(&profile_path).expect("Failed to create profile directory");

    Profile {
        name,
        path: profile_path,
    }
}

fn switch_profile(path: String) {
    let mut tmp = fs::File::create("/tmp/ccswitch_be").expect("Failed to create tmp file");
    write!(tmp, "{}", path).expect("Failed to write profile path");
}

fn main() {
    let args = Args::parse();

    let profiles = get_profiles();
    if args.success_to_switch {
        let claude_config_dir =
            env::var("CLAUDE_CONFIG_DIR").expect("Failed to get CLAUDE_CONFIG_DIR");
        let profile = profiles
            .iter()
            .find(|p| p.path == claude_config_dir)
            .unwrap();

        println!(
            "Switched to {} {}",
            style(&profile.name).blue().bold(),
            style(format!("({})", &profile.path)).dim()
        );

        return;
    }

    if let Some(profile_name) = args.profile {
        let profile = profiles
            .iter()
            .find(|p| p.name == profile_name)
            .expect(&format!("Profile {} is not found", profile_name));
        switch_profile(profile.path.clone());

        return;
    }

    let mut profile_names = profiles
        .iter()
        .map(|p| {
            format!(
                "{} {}",
                style(p.name.clone()).blue().bold(),
                style(p.path.clone()).dim()
            )
        })
        .collect::<Vec<String>>();
    let create_profile_option = style("+ Create new Profile").cyan().to_string();
    profile_names.push(create_profile_option.clone());

    let selected_profile = Select::new("Select a profile", profile_names.clone())
        .prompt()
        .expect("Failed to prompt for profile selection");

    if selected_profile == create_profile_option {
        let name = Text::new("Enter profile name:")
            .prompt()
            .expect("Failed to prompt for profile name");
        let profile = create_profile(name);
        switch_profile(profile.path);
    } else {
        let profile_index = profile_names
            .iter()
            .position(|p| p == &selected_profile)
            .expect("Failed to find selected profile");
        let profile = profiles[profile_index].clone();

        switch_profile(profile.path);
    }
}
