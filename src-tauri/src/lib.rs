use serde::{Deserialize, Serialize};
use std::env;
use sysinfo::{Disk, Disks, Networks, Pid, System};

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

#[derive(Serialize, Deserialize, PartialEq, PartialOrd)]
struct ProccessInfo {
    id: String,
    name: String,
    cpu: f32,
    memory: u64,
    disk_read: u64,
    disk_write: u64,
}

#[tauri::command]
fn process_info(name: Option<String>) -> Vec<ProccessInfo> {
    let mut sys = System::new();
    sys.refresh_all();

    let mut processes: Vec<ProccessInfo> = sys
        .processes()
        .iter()
        .map(|(id, process)| ProccessInfo {
            id: id.to_string(),
            name: process.name().to_string_lossy().into_owned(),
            cpu: process.cpu_usage(),
            memory: process.memory() / 1024 / 1024,
            disk_read: process.disk_usage().read_bytes / 1024 / 1024,
            disk_write: process.disk_usage().written_bytes / 1024 / 1024,
        })
        .filter(|proc| {
            if let Some(name) = &name {
                proc.name.starts_with(&*name)
            } else {
                true
            }
        })
        .collect();
    processes.sort_by_key(|e| e.name.clone());
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
    result
}

fn format_memory_size(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;
    const TB: f64 = GB * 1024.0;

    let size = bytes as f64;
    let (value, unit) = if size >= TB {
        (size / TB, "TB")
    } else if size >= GB {
        (size / GB, "GB")
    } else if size >= MB {
        (size / MB, "MB")
    } else if size >= KB {
        (size / KB, "KB")
    } else {
        (size, "bytes")
    };

    format!("{:.2} {}", value, unit)
}

#[derive(Serialize)]
struct DiskInfo {
    device: String,
    directory: String,
    r#type: String,
    total: String,
    available: String,
    used: String,
}

#[tauri::command]
fn file_systems() -> Vec<DiskInfo> {
    let disks = Disks::new_with_refreshed_list();
    let mut file_systems = Vec::new();
    for disk in &disks {
        let file_system = DiskInfo {
            device: disk.name().to_string_lossy().to_string(),
            directory: disk.mount_point().to_string_lossy().to_string(),
            r#type: disk.file_system().to_string_lossy().to_string(),
            total: format_memory_size(disk.total_space()),
            available: format_memory_size(disk.available_space()),
            used: format_memory_size(disk.total_space() - disk.available_space()),
        };
        file_systems.push(file_system);
    }
    file_systems
}

#[derive(Serialize)]
struct SysMetrics {
    cpus: Vec<f32>,
    used_memory: String,
    total_memory: String,
    free_memory: String,
    used_swap: String,
    total_swap: String,
    free_swap: String,
    //network
    recived: u64,
    total_recived: u64,
    transmited: u64,
    total_transmited: u64,
    // disk
}

#[tauri::command]
fn get_metrics() -> SysMetrics {
    let mut sys = System::new();
    sys.refresh_all();

    let network = Networks::new_with_refreshed_list();

    SysMetrics {
        cpus: sys.cpus().iter().map(|c| c.cpu_usage()).collect(),
        used_memory: format_memory_size(sys.used_memory()),
        total_memory: format_memory_size(sys.total_memory()),
        free_memory: format_memory_size(sys.free_memory()),
        used_swap: format_memory_size(sys.used_swap()),
        total_swap: format_memory_size(sys.used_swap()),
        free_swap: format_memory_size(sys.free_swap()),
        recived: network.iter().map(|(name, iface)| iface.received()).sum(),
        total_recived: network
            .iter()
            .map(|(name, iface)| iface.total_received())
            .sum(),
        transmited: network
            .iter()
            .map(|(name, iface)| iface.transmitted())
            .sum(),
        total_transmited: network
            .iter()
            .map(|(name, iface)| iface.total_transmitted())
            .sum(),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            os_info,
            process_info,
            kill_process,
            file_systems,
            get_metrics
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
