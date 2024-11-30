use std::{fs, process::Command};

use serde::{Deserialize, Serialize};

/// Abstracts an action that is performed when a button is clicked.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Action {
    /// Poweroff action.
    Poweroff,
    /// Reboot action.
    Reboot,
    /// Boot into Windows action.
    BootWindows(u8),
}

impl Action {
    pub fn perform(self) {
        let cmd = match self {
            Self::Poweroff => String::from("poweroff"),
            Self::Reboot => String::from("reboot"),
            Self::BootWindows(entry) => format!("sudo grub-reboot {entry} && reboot"),
        };

        Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .expect("failed to execute action");
    }
}

/// Config for the project.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    /// Title of the window.
    pub title: String,
    /// Width of the window.
    pub width: i32,
    /// Height of the window.
    pub height: i32,
    /// Keybind for poweroff button.
    pub bindp: String,
    /// Keybind for reboot button.
    pub bindr: String,
    /// Keybind for bootwin button.
    pub bindw: String,
    /// Windows boot entry (replaces $ENTRY variable).
    pub wbentry: u8,
}

impl Default for Config {
    /// Returns the default config.
    fn default() -> Self {
        ron::from_str(include_str!("../config/config.ron")).unwrap()
    }
}

impl Config {
    /// Reads the config from `$HOME/.config/poweroff-rs/config.ron`.
    pub fn load() -> Self {
        let Some(conf) = std::env::var("HOME")
            .ok()
            .and_then(|home| fs::read(home + "/.config/poweroff-rs/config.ron").ok())
        else {
            println!("Config not found! Falling back to default config.");
            return Config::default();
        };
        ron::from_str(String::from_utf8(conf).unwrap().as_str()).unwrap_or(Config::default())
    }
}
