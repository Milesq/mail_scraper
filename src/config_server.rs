use std::{process::Command, thread, time::Duration};

#[no_mangle]
pub extern "C" fn config(on_close: extern "C" fn()) {
    if !cfg!(debug_assertions) {
        Command::new("cmd")
            .args(&["/C", "start"])
            .arg(format!("http://localhost:{}", crate::CONFIG_SERVER_PORT))
            .output()
            .unwrap();
    }

    web_server::new()
        .get("/", Box::new(|_, _| "asd".into()))
        .get(
            "/close",
            Box::new(move |_, _| {
                println!("ok");
                thread::spawn(move || {
                    thread::sleep(Duration::from_millis(600));
                    on_close();
                });
                "asd".into()
            }),
        )
        .launch(crate::CONFIG_SERVER_PORT);
}
