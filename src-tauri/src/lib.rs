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
    ])
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
