mod archive_ops;
mod cargo_build;
mod defender;
mod git_ops;
mod powershell;
mod robocopy;
mod windows_search;

pub use archive_ops::ArchiveOpsBenchmark;
pub use cargo_build::CargoBuildBenchmark;
pub use defender::DefenderImpactBenchmark;
pub use git_ops::GitOperationsBenchmark;
pub use powershell::PowerShellBenchmark;
pub use robocopy::RobocopyBenchmark;
pub use windows_search::WindowsSearchBenchmark;
