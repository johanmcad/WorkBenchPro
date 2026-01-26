//! Process spawning utilities with hidden windows on Windows.

use std::process::{Command, Stdio};

#[cfg(windows)]
use std::os::windows::process::CommandExt as WinCommandExt;

/// Windows flag to create process without a console window
#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

/// Extension trait to configure commands to run without visible windows.
pub trait CommandExt {
    /// Configure the command to suppress console window visibility.
    fn hidden(&mut self) -> &mut Self;
}

impl CommandExt for Command {
    fn hidden(&mut self) -> &mut Self {
        // Redirect all standard streams
        self.stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        // On Windows, also set CREATE_NO_WINDOW flag to hide console
        #[cfg(windows)]
        {
            self.creation_flags(CREATE_NO_WINDOW);
        }

        self
    }
}

/// Create a new Command configured to run hidden.
pub fn hidden_command(program: &str) -> Command {
    let mut cmd = Command::new(program);
    cmd.hidden();
    cmd
}

/// Get full path to a Windows system executable.
/// Uses %WINDIR%\System32 to avoid PATH issues when SwiftShader env vars are set.
#[cfg(windows)]
pub fn system32_path(exe: &str) -> String {
    if let Ok(windir) = std::env::var("WINDIR") {
        format!(r"{}\System32\{}", windir, exe)
    } else {
        format!(r"C:\Windows\System32\{}", exe)
    }
}

#[cfg(not(windows))]
pub fn system32_path(exe: &str) -> String {
    exe.to_string()
}

/// Create a Command for a Windows system utility using full path.
/// The command is configured to run without a visible console window.
pub fn system_command(exe: &str) -> Command {
    let mut cmd = Command::new(system32_path(exe));
    cmd.hidden();
    cmd
}
