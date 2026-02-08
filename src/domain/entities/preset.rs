/// Represents a pre-configured profile preset
#[derive(Debug, Clone)]
pub struct ProfilePreset {
    pub name: &'static str,
    pub description: &'static str,
    pub theme: &'static str,
    pub plugins: &'static [&'static str],
    pub include_aliases: bool,
}

impl ProfilePreset {
    pub const MINIMAL: ProfilePreset = ProfilePreset {
        name: "Minimal",
        description: "Basic setup with essential features only",
        theme: "pure",
        plugins: &["PSReadLine", "posh-git"],
        include_aliases: false,
    };

    pub const DEVELOPER: ProfilePreset = ProfilePreset {
        name: "Developer",
        description: "Full-featured setup for developers",
        theme: "paradox",
        plugins: &["PSReadLine", "posh-git", "Terminal-Icons", "PSFzf", "z"],
        include_aliases: true,
    };

    pub const WORK: ProfilePreset = ProfilePreset {
        name: "Work",
        description: "Professional setup with productivity tools",
        theme: "jandedobbeleer",
        plugins: &["PSReadLine", "posh-git", "Terminal-Icons", "PSFzf"],
        include_aliases: true,
    };

    pub const CUSTOM: ProfilePreset = ProfilePreset {
        name: "Custom",
        description: "Choose your own theme and plugins",
        theme: "",
        plugins: &[],
        include_aliases: true,
    };

    pub fn all() -> &'static [ProfilePreset] {
        &[Self::MINIMAL, Self::DEVELOPER, Self::WORK, Self::CUSTOM]
    }
}
