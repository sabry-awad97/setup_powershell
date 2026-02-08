use typed_builder::TypedBuilder;

/// Represents a PowerShell profile configuration
#[derive(Debug, Clone, TypedBuilder)]
pub struct ProfileConfig {
    pub theme: String,
    pub plugins: Vec<String>,
    #[builder(default = true)]
    pub include_aliases: bool,
}
