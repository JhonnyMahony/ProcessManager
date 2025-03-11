use serde::{Deserialize, Serialize};
use std::env;
use sysinfo::{Pid, System};

#[derive(Serialize, Deserialize)]
struct SystemInfo {
    name: String,
}

#[tauri::command]
fn os_info() -> SystemInfo {
    SystemInfo {
        name: env::consts::OS.to_string(),
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
struct ProccessInfo {
    id: String,
    name: String,
    cpu: String,
    memory: String,
    disk_read: String,
    disk_write: String,
}

#[tauri::command]
fn process_info(name: Option<String>) -> Vec<ProccessInfo> {
    println!("{:?}", name);
    let mut sys = System::new();
    sys.refresh_all();

    let mut processes: Vec<ProccessInfo> = sys
        .processes()
        .iter()
        .map(|(id, process)| ProccessInfo {
            id: id.to_string(),
            name: process.name().to_string_lossy().into_owned(),
            cpu: process.cpu_usage().to_string(),
            memory: process.memory().to_string(),
            disk_read: process.disk_usage().read_bytes.to_string(),
            disk_write: process.disk_usage().written_bytes.to_string(),
        })
        .filter(|proc| {
            if let Some(name) = &name {
                proc.name.starts_with(&*name)
            } else {
                true
            }
        })
        .collect();
    processes.sort();
    processes
}

#[tauri::command]
fn kill_process(id: usize) -> Option<bool> {
    let mut sys = System::new();
    sys.refresh_all();
    let result = sys
        .processes()
        .get(&Pid::from(id))
        .map(|process| process.kill());
    println!("{:?}", result);

    result
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            os_info,
            process_info,
            kill_process
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
