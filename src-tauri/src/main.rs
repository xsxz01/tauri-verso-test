// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri_runtime_verso::{VersoRuntime, INVOKE_SYSTEM_SCRIPTS};
use tauri_verso_test_lib::test_handle;
fn main() {
    tauri::Builder::<VersoRuntime>::new()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![test_handle::greet])
        .invoke_system(INVOKE_SYSTEM_SCRIPTS)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
