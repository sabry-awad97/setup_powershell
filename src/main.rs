use anyhow::Result;
use std::sync::Arc;

use setup_powershell::domain::interfaces::{Installer, ShellRunner};
use setup_powershell::infrastructure::{
    FontInstaller, FzfInstaller, HttpDownloader, ModuleInstaller, OhMyPoshInstaller,
    PowerShellRunner, ProfileFsWriter, PwshInstaller, WindowsTerminalConfig,
};
use setup_powershell::{SetupCli, SetupService};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = SetupCli::new();
    cli.show_banner();

    // Determine which shell to use
    let use_pwsh = if !PowerShellRunner::pwsh().exists("pwsh").await {
        if cli.prompt_install_pwsh()? {
            // Install PowerShell 7
            let downloader = Arc::new(HttpDownloader::new());
            let pwsh_installer = PwshInstaller::new(downloader);
            pwsh_installer.install().await?;

            if !PowerShellRunner::pwsh().exists("pwsh").await {
                println!("\n⚠ PowerShell 7 installed but not available yet.");
                println!("Please restart your terminal and run this program again.\n");
                return Ok(());
            }
            true
        } else if PowerShellRunner::powershell().exists("powershell").await {
            println!("\n🔄 Continuing with Windows PowerShell...\n");
            false
        } else {
            anyhow::bail!("No PowerShell version found");
        }
    } else {
        true
    };

    // Select profile
    let preset = cli.select_preset()?;
    let config = cli.build_config_from_preset(preset)?;

    // Build dependencies
    let shell = Arc::new(if use_pwsh {
        PowerShellRunner::pwsh()
    } else {
        PowerShellRunner::powershell()
    });

    let downloader = Arc::new(HttpDownloader::new());
    let pwsh_installer = Arc::new(PwshInstaller::new(downloader.clone())) as Arc<dyn Installer>;
    let font_installer = Arc::new(FontInstaller::meslo()) as Arc<dyn Installer>;
    let terminal_config = Arc::new(WindowsTerminalConfig::new());
    let profile_writer = Arc::new(ProfileFsWriter::new(shell.clone()));

    // Build module installers
    let mut module_installers: Vec<Arc<dyn Installer>> = vec![];

    // Add oh-my-posh installer first
    module_installers.push(Arc::new(OhMyPoshInstaller::new()));

    for plugin in &config.plugins {
        let installer: Arc<dyn Installer> = match plugin.as_str() {
            "PSFzf" => {
                // PSFzf requires fzf, so add both
                module_installers.push(Arc::new(FzfInstaller::new()));
                Arc::new(ModuleInstaller::new(plugin.clone(), shell.clone()))
            }
            _ => Arc::new(ModuleInstaller::new(plugin.clone(), shell.clone())),
        };
        module_installers.push(installer);
    }

    // Build service
    let service = SetupService::builder()
        .shell(shell)
        .pwsh_installer(pwsh_installer)
        .font_installer(font_installer)
        .terminal_config(terminal_config)
        .profile_writer(profile_writer)
        .module_installers(module_installers)
        .build();

    // Run setup
    service.run_setup(&config).await?;

    Ok(())
}
