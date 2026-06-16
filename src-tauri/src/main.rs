use std::process::Command;

#[tauri::command]

fn start_nginx() -> String {

    let result = Command::new("systemctl")

        .args(["start", "nginx"])

        .output();

    match result {

        Ok(_) => "nginx 启动成功".into(),

        Err(e) => format!("失败: {}", e),

    }

}

fn main() {

    tauri::Builder::default()

        .invoke_handler(tauri::generate_handler![start_nginx])

        .run(tauri::generate_context!())

        .expect("error");

}
