use std::process::Command;

#[no_mangle]
pub extern "C" fn config() {
    println!("Server is online");

    Command::new("cmd")
        .args(&["/C", "start"])
        .arg(format!("http://localhost:{}", crate::CONFIG_SERVER_PORT))
        .output()
        .unwrap();
}
