use std::process::Command;

#[no_mangle]
pub extern "C" fn config() {
    println!("Server is online");

    Command::new("start")
        .arg(crate::CONFIG_SERVER_PORT)
        .output()
        .unwrap();
}
