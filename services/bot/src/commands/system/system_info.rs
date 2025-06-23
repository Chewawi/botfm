use crate::core::structs::{Context, Error};
use humanize_bytes::humanize_bytes_decimal as humanize;
use lumi::serenity_prelude as serenity;
use std::fmt::Write;
use std::time::Duration;
use sysinfo::{Disks, Networks, Pid, System};

#[lumi::command(slash_command, prefix_command)]
pub async fn system_info(ctx: Context<'_>) -> Result<(), Error> {
    let _ = ctx.defer_or_broadcast().await;
    let data = ctx.data();

    let mut sys = System::new();
    sys.refresh_all();

    let mut disks = Disks::new();
    disks.refresh(true);

    let mut networks = Networks::new();
    networks.refresh(true);

    let hostname = System::host_name().unwrap_or_else(|| "Unknown".into());
    let os_name = System::name().unwrap_or_else(|| "Unknown".into());
    let os_version = System::os_version().unwrap_or_else(|| "Unknown".into());
    let kernel_version = System::kernel_version().unwrap_or_else(|| "Unknown".into());

    let total_memory = humanize!(sys.total_memory());
    let used_memory = humanize!(sys.used_memory());
    let total_swap = humanize!(sys.total_swap());
    let used_swap = humanize!(sys.used_swap());

    let cpu_count = sys.cpus().len();
    let global_cpu = sys.global_cpu_usage();

    // Process
    let pid_u32 = std::process::id();
    let process_info = sys
        .process(Pid::from(pid_u32 as usize))
        .map(|p| {
            format!(
                "### Process Info\n\
            - Name: `{:?}`\n\
            -#\t- PID: `{}`\n\
            - RAM: `{}`\n\
            - CPU: `{:.2}%`\n\
            - Status: `{}`\n\
            - Uptime: `{}`",
                p.name(),
                p.pid().as_u32(),
                humanize!(p.memory()),
                p.cpu_usage(),
                p.status(),
                format_uptime(p.run_time()),
            )
        })
        .unwrap_or_else(|| "Process information not available".into());

    // Disks
    let disk_info = disks
        .list()
        .iter()
        .map(|d| {
            let used_pct = 100.0 * (1.0 - d.available_space() as f64 / d.total_space() as f64);
            format!(
                "**{:?}** (`{:?}`)\n- Usage: `{:.1}%` (`{}` / `{}`)",
                d.name(),
                d.file_system(),
                used_pct,
                humanize!(d.available_space()),
                humanize!(d.total_space()),
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    // Networks
    let (rx, tx) = networks.iter().fold((0u64, 0u64), |(r, t), (_iface, net)| {
        (r + net.received(), t + net.transmitted())
    });

    let bot_started = data.has_started.load(std::sync::atomic::Ordering::Relaxed);
    let bot_uptime = if bot_started {
        format_duration(data.time_started.elapsed())
    } else {
        "Bot not fully started yet".into() // idk, dont ask
    };
    let bot_user = ctx.cache().current_user().clone();
    let bot_info = format!(
        "### Bot Info\n\
        - Name: `{}`\n\
        -#\t- Started: `{}`\n\
        - Uptime: `{}`",
        bot_user.name, bot_started, bot_uptime,
    );

    // Generate ASCII tree view with all system information
    let ascii_tree = generate_ascii_tree(
        &hostname,
        &os_name,
        &os_version,
        &kernel_version,
        global_cpu,
        cpu_count,
        &used_memory,
        &total_memory,
        &used_swap,
        &total_swap,
        &process_info,
        &disk_info,
        rx,
        tx,
        &bot_info,
    );

    let container = serenity::CreateContainer::new(vec![serenity::CreateComponent::TextDisplay(
        serenity::CreateTextDisplay::new(ascii_tree),
    )]);

    ctx.send(
        lumi::CreateReply::default()
            .flags(serenity::MessageFlags::IS_COMPONENTS_V2)
            .components(&[serenity::CreateComponent::Container(container)]),
    )
    .await?;

    Ok(())
}

fn format_uptime(seconds: u64) -> String {
    let days = seconds / 86_400;
    let hours = (seconds % 86_400) / 3_600;
    let minutes = (seconds % 3_600) / 60;
    let secs = seconds % 60;
    format!("{days} days, {hours} hours, {minutes} minutes, {secs} seconds")
}

fn format_duration(duration: Duration) -> String {
    let s = duration.as_secs();
    format_uptime(s)
}

fn generate_ascii_tree(
    hostname: &str,
    os_name: &str,
    os_version: &str,
    kernel_version: &str,
    global_cpu: f32,
    cpu_count: usize,
    used_memory: &str,
    total_memory: &str,
    used_swap: &str,
    total_swap: &str,
    process_info: &str,
    disk_info: &str,
    rx: u64,
    tx: u64,
    bot_info: &str,
) -> String {
    let mut tree = String::new();

    // ANSI color codes - simplified
    let reset = "\u{001b}[0m";
    let bold = "\u{001b}[1m";
    let main = "\u{001b}[36m"; // cyan for tree structure and main elements
    let value = "\u{001b}[37m"; // white for values
    let label = "\u{001b}[37m"; // white for labels
    let red = "\u{001b}[31m"; // keep red for warnings/high usage
    let yellow = "\u{001b}[33m"; // keep yellow for medium usage
    let green = "\u{001b}[32m"; // keep green for low usage

    // System root
    writeln!(&mut tree, "{}{} System Information{}", bold, main, reset).unwrap();

    // Hostname and OS
    writeln!(
        &mut tree,
        "{}├── {}Hostname: {}{}{}{}",
        main, label, bold, value, hostname, reset
    )
    .unwrap();
    writeln!(&mut tree, "{}├── {}Operating System{}", main, label, reset).unwrap();
    writeln!(
        &mut tree,
        "{}│   ├── {}Name: {}{}{}{}",
        main, label, bold, value, os_name, reset
    )
    .unwrap();
    writeln!(
        &mut tree,
        "{}│   ├── {}Version: {}{}{}{}",
        main, label, bold, value, os_version, reset
    )
    .unwrap();
    writeln!(
        &mut tree,
        "{}│   └── {}Kernel: {}{}{}{}",
        main, label, bold, value, kernel_version, reset
    )
    .unwrap();

    // CPU
    writeln!(&mut tree, "{}├── {}CPU{}", main, label, reset).unwrap();
    writeln!(
        &mut tree,
        "{}│   ├── {}Count: {}{}{}{}",
        main, label, bold, value, cpu_count, reset
    )
    .unwrap();

    // Color CPU usage based on value
    let cpu_color = if global_cpu > 80.0 {
        red
    } else if global_cpu > 50.0 {
        yellow
    } else {
        green
    };
    writeln!(
        &mut tree,
        "{}│   └── {}Usage: {}{}{:.2}%{}",
        main, label, bold, cpu_color, global_cpu, reset
    )
    .unwrap();

    // Memory
    writeln!(&mut tree, "{}├── {}Memory{}", main, label, reset).unwrap();
    writeln!(
        &mut tree,
        "{}│   ├── {}RAM: {}{}{} / {}{}",
        main, label, bold, value, used_memory, total_memory, reset
    )
    .unwrap();
    writeln!(
        &mut tree,
        "{}│   └── {}Swap: {}{}{} / {}{}",
        main, label, bold, value, used_swap, total_swap, reset
    )
    .unwrap();

    // Network
    writeln!(&mut tree, "{}├── {}Network{}", main, label, reset).unwrap();
    writeln!(
        &mut tree,
        "{}│   ├── {}Received: {}{}{}{}",
        main,
        label,
        bold,
        value,
        humanize!(rx),
        reset
    )
    .unwrap();
    writeln!(
        &mut tree,
        "{}│   └── {}Transmitted: {}{}{}{}",
        main,
        label,
        bold,
        value,
        humanize!(tx),
        reset
    )
    .unwrap();

    // Process
    writeln!(&mut tree, "{}├── {}Process{}", main, label, reset).unwrap();
    // Extract process info from the formatted string
    if let Some(name_line) = process_info.lines().nth(1) {
        if let Some(name) = name_line.trim().strip_prefix("- Name: ") {
            writeln!(
                &mut tree,
                "{}│   ├── {}Name: {}{}{}{}",
                main,
                label,
                bold,
                value,
                name.trim_matches('`'),
                reset
            )
            .unwrap();
        }
    }
    if let Some(pid_line) = process_info.lines().nth(2) {
        if let Some(pid) = pid_line.trim().strip_prefix("-#\t- PID: ") {
            writeln!(
                &mut tree,
                "{}│   ├── {}PID: {}{}{}{}",
                main,
                label,
                bold,
                value,
                pid.trim_matches('`'),
                reset
            )
            .unwrap();
        }
    }
    if let Some(ram_line) = process_info.lines().nth(3) {
        if let Some(ram) = ram_line.trim().strip_prefix("- RAM: ") {
            writeln!(
                &mut tree,
                "{}│   ├── {}RAM: {}{}{}{}",
                main,
                label,
                bold,
                value,
                ram.trim_matches('`'),
                reset
            )
            .unwrap();
        }
    }
    if let Some(cpu_line) = process_info.lines().nth(4) {
        if let Some(cpu) = cpu_line.trim().strip_prefix("- CPU: ") {
            let cpu_val = cpu.trim_matches('`');
            let cpu_num = cpu_val.trim_end_matches('%').parse::<f32>().unwrap_or(0.0);
            let cpu_color = if cpu_num > 80.0 {
                red
            } else if cpu_num > 50.0 {
                yellow
            } else {
                green
            };
            writeln!(
                &mut tree,
                "{}│   ├── {}CPU: {}{}{}{}",
                main, label, bold, cpu_color, cpu_val, reset
            )
            .unwrap();
        }
    }
    if let Some(status_line) = process_info.lines().nth(5) {
        if let Some(status) = status_line.trim().strip_prefix("- Status: ") {
            writeln!(
                &mut tree,
                "{}│   ├── {}Status: {}{}{}{}",
                main,
                label,
                bold,
                value,
                status.trim_matches('`'),
                reset
            )
            .unwrap();
        }
    }
    if let Some(uptime_line) = process_info.lines().nth(6) {
        if let Some(uptime) = uptime_line.trim().strip_prefix("- Uptime: ") {
            writeln!(
                &mut tree,
                "{}│   └── {}Uptime: {}{}{}{}",
                main,
                label,
                bold,
                value,
                uptime.trim_matches('`'),
                reset
            )
            .unwrap();
        }
    }

    // Disk info
    writeln!(&mut tree, "{}├── {}Disks{}", main, label, reset).unwrap();

    // Parse disk info
    if disk_info.is_empty() {
        writeln!(&mut tree, "{}│   └── {}No disks found{}", main, red, reset).unwrap();
    } else {
        let disk_lines: Vec<&str> = disk_info.split("\n\n").collect();
        for (i, disk_line) in disk_lines.iter().enumerate() {
            let is_last = i == disk_lines.len() - 1;
            let prefix = if is_last {
                "│   └── "
            } else {
                "│   ├── "
            };

            // Extract disk name
            if let Some(first_line) = disk_line.lines().next() {
                if let Some(name_start) = first_line.find("**") {
                    if let Some(name_end) = first_line[name_start + 2..].find("**") {
                        let name = &first_line[name_start + 2..name_start + 2 + name_end];
                        writeln!(
                            &mut tree,
                            "{}{}{}{}Disk: {}{}{}{}",
                            main, prefix, bold, value, name, reset, main, reset
                        )
                        .unwrap();

                        // Extract filesystem
                        if let Some(fs_start) = first_line.find("(`") {
                            if let Some(fs_end) = first_line[fs_start + 2..].find("`)") {
                                let fs = &first_line[fs_start + 2..fs_start + 2 + fs_end];
                                writeln!(
                                    &mut tree,
                                    "{}│   {}    ├── {}Filesystem: {}{}{}{}",
                                    main, main, label, bold, value, fs, reset
                                )
                                .unwrap();
                            }
                        }

                        // Extract usage info
                        if let Some(second_line) = disk_line.lines().nth(1) {
                            if let Some(usage_start) = second_line.find("`") {
                                if let Some(usage_end) = second_line[usage_start + 1..].find("`") {
                                    let usage =
                                        &second_line[usage_start + 1..usage_start + 1 + usage_end];
                                    let usage_num =
                                        usage.trim_end_matches('%').parse::<f32>().unwrap_or(0.0);
                                    let usage_color = if usage_num > 90.0 {
                                        red
                                    } else if usage_num > 70.0 {
                                        yellow
                                    } else {
                                        green
                                    };
                                    writeln!(
                                        &mut tree,
                                        "{}│   {}    └── {}Usage: {}{}{}{}",
                                        main, main, label, bold, usage_color, usage, reset
                                    )
                                    .unwrap();
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Bot info
    writeln!(&mut tree, "{}└── {}Bot{}", main, label, reset).unwrap();
    // Extract bot info from the formatted string
    if let Some(name_line) = bot_info.lines().nth(1) {
        if let Some(name) = name_line.trim().strip_prefix("- Name: ") {
            writeln!(
                &mut tree,
                "{}    ├── {}Name: {}{}{}{}",
                main,
                label,
                bold,
                value,
                name.trim_matches('`'),
                reset
            )
            .unwrap();
        }
    }
    if let Some(started_line) = bot_info.lines().nth(2) {
        if let Some(started) = started_line.trim().strip_prefix("-#\t- Started: ") {
            writeln!(
                &mut tree,
                "{}    ├── {}Started: {}{}{}{}",
                main,
                label,
                bold,
                value,
                started.trim_matches('`'),
                reset
            )
            .unwrap();
        }
    }
    if let Some(uptime_line) = bot_info.lines().nth(3) {
        if let Some(uptime) = uptime_line.trim().strip_prefix("- Uptime: ") {
            writeln!(
                &mut tree,
                "{}    └── {}Uptime: {}{}{}{}",
                main,
                label,
                bold,
                value,
                uptime.trim_matches('`'),
                reset
            )
            .unwrap();
        }
    }

    // Wrap in a code block with ansi syntax highlighting for colors
    format!("```ansi\n{}\n```", tree)
}
