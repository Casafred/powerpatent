mod commands;
mod pdf;
mod table;
mod ai;
mod ocr;
mod render;
mod cache;
mod types;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_log::Builder::default().build())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_fs::init())
    .invoke_handler(tauri::generate_handler![
      commands::process_input,
      commands::extract_pdf_text,
      commands::extract_pdf_images,
      commands::map_fields,
      commands::ocr_pdf,
      commands::generate_module,
      commands::get_cached_module,
      commands::rerun_module,
      commands::render_html,
      commands::export_html,
      commands::test_ai_connection,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
