use std::io;
use std::process::Command;

fn run_cec_ctl(args: &[&str]) -> io::Result<()> {
    let output = Command::new("sudo").arg("cec-ctl").args(args).output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !stdout.is_empty() {
        log::info!("cec-ctl stdout: {}", stdout);
    }

    if !stderr.is_empty() {
        log::error!("cec-ctl stderr: {}", stderr);
    }

    if !output.status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "cec-ctl failed (exit={:?})\nargs={:?}\nstdout:\n{}\nstderr:\n{}",
                output.status.code(),
                args,
                stdout,
                stderr
            ),
        ));
    }

    Ok(())
}

pub fn register_playback(osd_name: &str) -> io::Result<()> {
    run_cec_ctl(&["--playback", "--osd-name", osd_name])
}

pub fn power_off() -> io::Result<()> {
    run_cec_ctl(&["--to", "0", "--standby"])
}

pub fn power_on() -> io::Result<()> {
    run_cec_ctl(&["--to", "0", "--user-control-pressed", "ui-cmd=power"])?;

    run_cec_ctl(&["--to", "0", "--user-control-released"])
}

pub fn volume_up() -> io::Result<()> {
    run_cec_ctl(&["--to", "0", "--user-control-pressed", "ui-cmd=volume-up"])?;

    run_cec_ctl(&["--to", "0", "--user-control-released"])
}

pub fn volume_down() -> io::Result<()> {
    run_cec_ctl(&["--to", "0", "--user-control-pressed", "ui-cmd=volume-down"])?;

    run_cec_ctl(&["--to", "0", "--user-control-released"])
}

pub fn toggle_mute() -> io::Result<()> {
    run_cec_ctl(&["--to", "0", "--user-control-pressed", "ui-cmd=mute"])?;

    run_cec_ctl(&["--to", "0", "--user-control-released"])
}

pub fn active_source(source_physical_address_number: u8) -> io::Result<()> {
    let physical_address_param = format!("phys-addr={}.0.0.0", source_physical_address_number);

    run_cec_ctl(&["--active-source", &physical_address_param])
}
