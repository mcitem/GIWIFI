use tauri::State;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState(giwifi::Client::default()))
        .invoke_handler(tauri::generate_handler![login, logout])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

struct AppState(pub giwifi::Client);

#[tauri::command]
async fn login(
    username: String,
    password: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    state
        .0
        .login(&username, &password)
        .await
        .map_err(|e| e.to_string())?;
    Ok("Login initiated".to_string())
}

#[tauri::command]
async fn logout(state: State<'_, AppState>) -> Result<String, String> {
    let s = state.0.logout().await.map_err(|e| e.to_string())?;
    if s {
        Ok("Logout successful".to_string())
    } else {
        Err("Logout failed".to_string())
    }
}
