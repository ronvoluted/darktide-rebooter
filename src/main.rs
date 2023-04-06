#![windows_subsystem = "windows"]

use chrono::Utc;
use std::env;
use std::fs;
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::{Command, Output, Stdio};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{io, thread};
use systray::Error;
use walkdir::WalkDir;
use winapi::um::winbase::CREATE_NO_WINDOW;

extern crate open;

const ACTIVE_ICON: &str = "rebooter";
const INACTIVE_ICON: &str = "rebooter-inactive";

fn main() -> Result<(), Error> {
    let mut app = systray::Application::new()?;
    app.set_icon_from_resource(ACTIVE_ICON)?;
    app.set_tooltip("Active")?;

    let monitoring_active = Arc::new(Mutex::new(true));

    let monitoring_active_clone = monitoring_active.clone();
    app.add_menu_item("Toggle monitoring", move |app| {
        let mut locked_monitoring_active = monitoring_active_clone.lock().unwrap();
        *locked_monitoring_active = !*locked_monitoring_active;

        if *locked_monitoring_active {
            app.set_icon_from_resource(ACTIVE_ICON)?;
            app.set_tooltip("Active")?;
        } else {
            app.set_icon_from_resource(INACTIVE_ICON)?;
            app.set_tooltip("Inactive")?;
        }

        Ok::<(), systray::Error>(())
    })?;

    app.add_menu_separator()?;

    app.add_menu_item("Darktide Rebooter", |_app| {
        let path = "https://github.com/ronvoluted/darktide-rebooter";

        match open::that(path) {
            Ok(()) => (),
            Err(err) => eprintln!("An error occurred when opening '{}': {}", path, err),
        }

        Ok::<(), systray::Error>(())
    })?;

    app.add_menu_separator()?;

    app.add_menu_item("Exit", |app| {
        app.quit();

        Ok::<(), systray::Error>(())
    })?;

    let monitoring_active_clone = monitoring_active.clone();

    thread::spawn(move || loop {
        let should_monitor = {
            let monitoring_active = monitoring_active_clone.lock().unwrap();
            *monitoring_active
        };

        if should_monitor {
            if let Err(e) = check_logs_and_run_executable() {
                eprintln!("Error: {}", e);
            }
        }

        thread::sleep(Duration::from_secs(1));
    });

    app.wait_for_message()?;

    Ok(())
}

fn check_logs_and_run_executable() -> io::Result<()> {
    let app_data = env::var("APPDATA").map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let logs_path = Path::new(&app_data).join("Fatshark\\Darktide\\crash_dumps");

    for entry in WalkDir::new(logs_path).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        let metadata = fs::metadata(path)?;

        if path
            .file_name()
            .and_then(|s| s.to_str())
            .filter(|s| s.starts_with("crash_dump"))
            .is_some()
        {
            let modified = metadata
                .modified()
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            let duration = modified
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            let now = Utc::now().timestamp() as u64;

            if now - duration.as_secs() <= 30 {
                wait_until_darktide_not_running()?;

                if !is_darktide_running()? {
                    run_darktide_executable()?;
                }
            }
        }
    }

    Ok(())
}

fn wait_until_darktide_not_running() -> io::Result<()> {
    loop {
        if !is_darktide_running()? {
            break;
        }

        thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}

fn is_darktide_running() -> io::Result<bool> {
    let output = get_tasklist_output()?;
    let task_list = String::from_utf8_lossy(&output.stdout);

    Ok(task_list.contains("Darktide.exe"))
}

fn get_tasklist_output() -> io::Result<Output> {
    Command::new("tasklist")
        .arg("/FI")
        .arg("IMAGENAME eq Darktide.exe")
        .stdout(Stdio::piped())
        .creation_flags(CREATE_NO_WINDOW)
        .output()
}

fn run_darktide_executable() -> io::Result<()> {
    let working_directory = env::current_dir()?;
    let executable_path = working_directory.join("binaries\\Darktide.exe");

    let mut command = Command::new(&executable_path);
    command
        .args(&[
            "--bundle-dir",
            "../bundle",
            "--ini",
            "settings",
            "--backend-auth-service-url",
            "https://bsp-auth-prod.atoma.cloud",
            "--backend-title-service-url",
            "https://bsp-td-prod.atoma.cloud",
        ])
        .env("SteamAppId", "1361210")
        .creation_flags(CREATE_NO_WINDOW);
    command.status()?;

    Ok(())
}
