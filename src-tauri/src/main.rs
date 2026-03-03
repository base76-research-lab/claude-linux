#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;

#[tauri::command]
async fn send_message(prompt: String) -> Result<String, String> {
    api::chat(prompt).await
}

fn main() {
    dotenv::dotenv().ok();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![send_message])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}