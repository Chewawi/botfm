use crate::{Context, Error};
use humanize_bytes::humanize_bytes_decimal as humanize;
use poise::command;
use poise::serenity_prelude as serenity;
use sysinfo::Disks;
use sysinfo::Pid;
use sysinfo::System;

#[command(slash_command, prefix_command)]
pub async fn system_info(ctx: Context<'_>) -> Result<(), Error> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let current_pid = std::process::id();
    let process = sys.process(Pid::from(current_pid as usize));

    let (process_info, disk_info, uptime_info) = match process {
        Some(p) => {
            let name = p.name();
            let pid = p.pid().as_u32();
            let memory_readable = humanize!(p.memory());
            let cpu_usage = p.cpu_usage();
            let status = p.status().to_string();

            let disks = Disks::new_with_refreshed_list();
            let disk = &disks.list()[0];

            let disk_info = format!(
                "\tName: `{:?}`\n\tType: `{:?}`\n\t* Usage: `{} / {}`",
                disk.name(),
                disk.file_system(),
                humanize!(disk.total_space()),
                humanize!(disk.available_space())
            );

            let uptime_info = format_uptime(p.run_time());

            (
                format!(
                    "
                * Process Name: `{:?}`\n\
                \t* PID: `{}`\n\
                * Memory Usage: {}\n\
                * CPU Usage: {:.2}%\n\
                * Status: {}\n\
                ",
                    name, pid, memory_readable, cpu_usage, status
                ),
                disk_info,
                uptime_info,
            )
        }
        _none => (
            "Process information not found".to_string(),
            "Disk info not found".to_string(),
            "".to_string(),
        ),
    };

    let u = ctx.cache().current_user().clone();
    let _ = ctx.defer_or_broadcast().await;

    let avatar = u.face();

    let embed = serenity::CreateEmbed::new()
        .title("System Info")
        .description(process_info)
        .fields(vec![
            ("Disk Info", disk_info, false),
            ("Uptime", uptime_info, false),
        ])
        .thumbnail(&avatar)
        .author(serenity::CreateEmbedAuthor::new(format!("@{}", u.name.as_str())).icon_url(&avatar))
        .color(serenity::Color::new(0x1D1D1D));

    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    Ok(())
}

fn format_uptime(seconds: u64) -> String {
    let days = seconds / (60 * 60 * 24);
    let hours = (seconds % (60 * 60 * 24)) / (60 * 60);
    let minutes = (seconds % (60 * 60)) / 60;
    let secs = seconds % 60;

    format!(
        "{} days, {} hours, {} minutes, {} seconds",
        days, hours, minutes, secs
    )
}
