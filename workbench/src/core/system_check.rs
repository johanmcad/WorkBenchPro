//! Pre-benchmark system check module.
//!
//! Checks system readiness before running benchmarks to ensure accurate results.

use std::process::Command;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, ProcessRefreshKind, RefreshKind, System};

#[cfg(windows)]
use windows::Win32::System::Power::{GetSystemPowerStatus, SYSTEM_POWER_STATUS};

/// Severity level for system warnings
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WarningSeverity {
    /// Informational - may affect results slightly
    Info,
    /// Warning - will likely affect benchmark accuracy
    Warning,
    /// Critical - benchmark results will be unreliable
    Critical,
}

impl WarningSeverity {
    /// Get a display label for the severity
    pub fn label(&self) -> &'static str {
        match self {
            WarningSeverity::Info => "Info",
            WarningSeverity::Warning => "Warning",
            WarningSeverity::Critical => "Critical",
        }
    }
}

/// Power state of the system
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PowerState {
    /// Plugged in to AC power
    PluggedIn,
    /// Running on battery with given percentage remaining
    OnBattery(u8),
    /// Unable to determine power state
    Unknown,
}

impl PowerState {
    /// Check if running on battery
    pub fn is_on_battery(&self) -> bool {
        matches!(self, PowerState::OnBattery(_))
    }

    /// Get battery percentage if on battery
    pub fn battery_percent(&self) -> Option<u8> {
        match self {
            PowerState::OnBattery(pct) => Some(*pct),
            _ => None,
        }
    }
}

/// Windows power plan
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PowerPlan {
    /// High Performance mode
    HighPerformance,
    /// Balanced mode (default)
    Balanced,
    /// Power Saver mode
    PowerSaver,
    /// Unknown or custom power plan
    Unknown(String),
}

impl PowerPlan {
    /// Check if this is a suboptimal power plan for benchmarking
    pub fn is_suboptimal(&self) -> bool {
        matches!(self, PowerPlan::Balanced | PowerPlan::PowerSaver | PowerPlan::Unknown(_))
    }

    /// Get display label
    pub fn label(&self) -> &str {
        match self {
            PowerPlan::HighPerformance => "High Performance",
            PowerPlan::Balanced => "Balanced",
            PowerPlan::PowerSaver => "Power Saver",
            PowerPlan::Unknown(name) => name,
        }
    }
}

/// Information about a high-CPU process
#[derive(Debug, Clone)]
pub struct ProcessInfo {
    /// Process name
    pub name: String,
    /// Process ID
    pub pid: u32,
    /// CPU usage percentage
    pub cpu_percent: f32,
}

/// A warning about system state that may affect benchmark results
#[derive(Debug, Clone)]
pub struct SystemWarning {
    /// Severity of the warning
    pub severity: WarningSeverity,
    /// Short title for the warning
    pub title: String,
    /// Detailed description
    pub description: String,
    /// Optional remediation steps
    pub remediation: Option<String>,
}

/// Result of the system check
#[derive(Debug, Clone)]
pub struct SystemCheckResult {
    /// Current CPU usage as a percentage (0-100)
    pub cpu_usage_percent: f32,
    /// Available memory in GB
    pub available_memory_gb: f64,
    /// Current power state
    pub power_state: PowerState,
    /// Current power plan
    pub power_plan: PowerPlan,
    /// List of processes using significant CPU
    pub high_cpu_processes: Vec<ProcessInfo>,
    /// Generated warnings based on checks
    pub warnings: Vec<SystemWarning>,
    /// Whether the system is ready for benchmarking
    pub ready_to_benchmark: bool,
}

/// System checker that performs pre-benchmark readiness checks
pub struct SystemChecker;

impl SystemChecker {
    /// Perform all system checks and return the result
    pub fn check() -> SystemCheckResult {
        let mut sys = System::new_with_specifics(
            RefreshKind::new()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything())
                .with_processes(ProcessRefreshKind::everything()),
        );

        // Sample CPU usage over 3 seconds for accurate average
        // First refresh to initialize
        sys.refresh_all();
        std::thread::sleep(std::time::Duration::from_millis(500));

        // Collect CPU samples every 500ms for 2.5 seconds (5 samples)
        let mut cpu_samples = Vec::with_capacity(5);
        for _ in 0..5 {
            sys.refresh_cpu_usage();
            cpu_samples.push(Self::get_cpu_usage(&sys));
            std::thread::sleep(std::time::Duration::from_millis(500));
        }

        // Final refresh for memory and processes
        sys.refresh_all();

        // Calculate average CPU usage
        let cpu_usage_percent = cpu_samples.iter().sum::<f32>() / cpu_samples.len() as f32;
        let available_memory_gb = Self::get_available_memory(&sys);
        let power_state = Self::get_power_state();
        let power_plan = Self::get_power_plan();
        let high_cpu_processes = Self::get_high_cpu_processes(&sys);

        let mut warnings = Vec::new();

        // Check CPU usage
        if cpu_usage_percent > 50.0 {
            warnings.push(SystemWarning {
                severity: WarningSeverity::Critical,
                title: "High CPU Usage".to_string(),
                description: format!(
                    "CPU usage is {:.0}%. Background tasks will significantly skew results.",
                    cpu_usage_percent
                ),
                remediation: Some(
                    "Close unnecessary applications and background tasks before benchmarking."
                        .to_string(),
                ),
            });
        } else if cpu_usage_percent > 20.0 {
            warnings.push(SystemWarning {
                severity: WarningSeverity::Warning,
                title: "Elevated CPU Usage".to_string(),
                description: format!(
                    "CPU usage is {:.0}%. Background tasks may affect results.",
                    cpu_usage_percent
                ),
                remediation: Some(
                    "Consider closing background applications for more accurate results.".to_string(),
                ),
            });
        }

        // Check available memory
        if available_memory_gb < 2.0 {
            warnings.push(SystemWarning {
                severity: WarningSeverity::Critical,
                title: "Very Low Memory".to_string(),
                description: format!(
                    "Only {:.1}GB of memory available. Benchmarks need working space.",
                    available_memory_gb
                ),
                remediation: Some("Close memory-intensive applications before benchmarking.".to_string()),
            });
        } else if available_memory_gb < 4.0 {
            warnings.push(SystemWarning {
                severity: WarningSeverity::Warning,
                title: "Low Memory".to_string(),
                description: format!(
                    "Only {:.1}GB of memory available. Some benchmarks may be affected.",
                    available_memory_gb
                ),
                remediation: Some("Consider closing some applications for optimal results.".to_string()),
            });
        }

        // Check power state
        match power_state {
            PowerState::OnBattery(pct) if pct < 50 => {
                warnings.push(SystemWarning {
                    severity: WarningSeverity::Critical,
                    title: "Low Battery".to_string(),
                    description: format!(
                        "Running on battery at {}%. CPU may throttle to save power.",
                        pct
                    ),
                    remediation: Some("Connect to AC power for accurate benchmark results.".to_string()),
                });
            }
            PowerState::OnBattery(pct) if pct < 80 => {
                warnings.push(SystemWarning {
                    severity: WarningSeverity::Warning,
                    title: "On Battery Power".to_string(),
                    description: format!(
                        "Running on battery at {}%. Performance may be limited.",
                        pct
                    ),
                    remediation: Some(
                        "Connect to AC power for optimal performance during benchmarking.".to_string(),
                    ),
                });
            }
            PowerState::OnBattery(_) => {
                warnings.push(SystemWarning {
                    severity: WarningSeverity::Info,
                    title: "On Battery Power".to_string(),
                    description: "Running on battery. Performance may be limited.".to_string(),
                    remediation: Some(
                        "Connect to AC power for optimal performance during benchmarking.".to_string(),
                    ),
                });
            }
            _ => {}
        }

        // Check power plan
        match &power_plan {
            PowerPlan::PowerSaver => {
                warnings.push(SystemWarning {
                    severity: WarningSeverity::Critical,
                    title: "Power Saver Mode".to_string(),
                    description: "Power Saver mode limits CPU frequency significantly.".to_string(),
                    remediation: Some(
                        "Switch to High Performance or Balanced power plan: \
                         Settings > System > Power & battery > Power mode".to_string(),
                    ),
                });
            }
            PowerPlan::Balanced => {
                warnings.push(SystemWarning {
                    severity: WarningSeverity::Info,
                    title: "Balanced Power Plan".to_string(),
                    description: "Balanced mode may limit peak CPU performance.".to_string(),
                    remediation: Some(
                        "For maximum performance, consider switching to High Performance mode."
                            .to_string(),
                    ),
                });
            }
            _ => {}
        }

        // Check for high-CPU processes
        let high_cpu: Vec<_> = high_cpu_processes.iter().filter(|p| p.cpu_percent > 25.0).collect();
        if !high_cpu.is_empty() {
            let process_names: Vec<_> = high_cpu.iter().map(|p| p.name.clone()).collect();
            warnings.push(SystemWarning {
                severity: WarningSeverity::Critical,
                title: "High-CPU Processes".to_string(),
                description: format!(
                    "Processes using >25% CPU: {}",
                    process_names.join(", ")
                ),
                remediation: Some("Close or wait for these processes to complete.".to_string()),
            });
        } else {
            let elevated_cpu: Vec<_> = high_cpu_processes.iter().filter(|p| p.cpu_percent > 10.0).collect();
            if !elevated_cpu.is_empty() {
                let process_names: Vec<_> = elevated_cpu.iter().map(|p| p.name.clone()).collect();
                warnings.push(SystemWarning {
                    severity: WarningSeverity::Warning,
                    title: "Active Processes".to_string(),
                    description: format!(
                        "Processes using >10% CPU: {}",
                        process_names.join(", ")
                    ),
                    remediation: Some(
                        "Consider closing these processes for more accurate results.".to_string(),
                    ),
                });
            }
        }

        // Determine if ready to benchmark (no critical warnings)
        let ready_to_benchmark = !warnings.iter().any(|w| w.severity == WarningSeverity::Critical);

        SystemCheckResult {
            cpu_usage_percent,
            available_memory_gb,
            power_state,
            power_plan,
            high_cpu_processes,
            warnings,
            ready_to_benchmark,
        }
    }

    fn get_cpu_usage(sys: &System) -> f32 {
        let cpus = sys.cpus();
        if cpus.is_empty() {
            return 0.0;
        }
        cpus.iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / cpus.len() as f32
    }

    fn get_available_memory(sys: &System) -> f64 {
        sys.available_memory() as f64 / (1024.0 * 1024.0 * 1024.0)
    }

    #[cfg(windows)]
    fn get_power_state() -> PowerState {
        let mut status = SYSTEM_POWER_STATUS::default();
        unsafe {
            if GetSystemPowerStatus(&mut status).is_ok() {
                // ACLineStatus: 0 = offline, 1 = online, 255 = unknown
                if status.ACLineStatus == 1 {
                    return PowerState::PluggedIn;
                } else if status.ACLineStatus == 0 {
                    // BatteryLifePercent: 0-100 or 255 for unknown
                    let percent = if status.BatteryLifePercent == 255 {
                        50 // Default to 50% if unknown
                    } else {
                        status.BatteryLifePercent
                    };
                    return PowerState::OnBattery(percent);
                }
            }
        }
        PowerState::Unknown
    }

    #[cfg(not(windows))]
    fn get_power_state() -> PowerState {
        PowerState::Unknown
    }

    #[cfg(windows)]
    fn get_power_plan() -> PowerPlan {
        // Use powercfg to get active power scheme
        let output = Command::new("powercfg")
            .args(["/getactivescheme"])
            .output();

        if let Ok(output) = output {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stdout_lower = stdout.to_lowercase();

            if stdout_lower.contains("high performance") {
                return PowerPlan::HighPerformance;
            } else if stdout_lower.contains("power saver") {
                return PowerPlan::PowerSaver;
            } else if stdout_lower.contains("balanced") {
                return PowerPlan::Balanced;
            }

            // Try to extract the plan name from the output
            // Format: "Power Scheme GUID: xxxxxxxx  (Plan Name)"
            if let Some(start) = stdout.find('(') {
                if let Some(end) = stdout.find(')') {
                    let name = stdout[start + 1..end].trim().to_string();
                    return PowerPlan::Unknown(name);
                }
            }
        }

        PowerPlan::Unknown("Unknown".to_string())
    }

    #[cfg(not(windows))]
    fn get_power_plan() -> PowerPlan {
        PowerPlan::Unknown("N/A".to_string())
    }

    fn get_high_cpu_processes(sys: &System) -> Vec<ProcessInfo> {
        // Get our own PID to exclude from the list
        let our_pid = std::process::id();

        let mut processes: Vec<ProcessInfo> = sys
            .processes()
            .iter()
            .filter_map(|(pid, process)| {
                // Skip ourselves
                if pid.as_u32() == our_pid {
                    return None;
                }

                let cpu = process.cpu_usage();
                if cpu > 5.0 {
                    // Only include processes using more than 5% CPU
                    Some(ProcessInfo {
                        name: process.name().to_string_lossy().to_string(),
                        pid: pid.as_u32(),
                        cpu_percent: cpu,
                    })
                } else {
                    None
                }
            })
            .collect();

        // Sort by CPU usage descending
        processes.sort_by(|a, b| b.cpu_percent.partial_cmp(&a.cpu_percent).unwrap_or(std::cmp::Ordering::Equal));

        // Return top 10
        processes.truncate(10);
        processes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_check() {
        let result = SystemChecker::check();

        // Basic sanity checks
        assert!(result.cpu_usage_percent >= 0.0);
        assert!(result.cpu_usage_percent <= 100.0);
        assert!(result.available_memory_gb >= 0.0);
    }

    #[test]
    fn test_warning_severity() {
        assert_eq!(WarningSeverity::Info.label(), "Info");
        assert_eq!(WarningSeverity::Warning.label(), "Warning");
        assert_eq!(WarningSeverity::Critical.label(), "Critical");
    }

    #[test]
    fn test_power_state() {
        assert!(!PowerState::PluggedIn.is_on_battery());
        assert!(PowerState::OnBattery(50).is_on_battery());
        assert_eq!(PowerState::OnBattery(75).battery_percent(), Some(75));
        assert_eq!(PowerState::PluggedIn.battery_percent(), None);
    }
}
