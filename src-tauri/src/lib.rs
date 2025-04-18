use tauri_runtime_verso::{VersoRuntime, INVOKE_SYSTEM_SCRIPTS};
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

pub mod test_handle;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::<VersoRuntime>::new()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![test_handle::greet])
        .invoke_system(INVOKE_SYSTEM_SCRIPTS)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

