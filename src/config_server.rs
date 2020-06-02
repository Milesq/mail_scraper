use std::{env, process::Command, thread, time::Duration};
use web_server::{HttpCode, Request, Response};

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
                thread::spawn(move || {
                    thread::sleep(Duration::from_millis(600));
                    on_close();
                });
                "asd".into()
            }),
        )
        .not_found(Box::new(|req: Request, mut default_resp: Response| {
            let mut path = env::current_exe()
                .expect("Cannot find current exe")
                .parent()
                .unwrap()
                .to_path_buf();

            path.push(format!("./static{}", req.get_path()));

            if path.exists() {
                path.as_path().into()
            } else {
                default_resp
                    .set_http_code(HttpCode::_404)
                    .set_body("Nie znaleziono pliku");
                default_resp
            }
        }))
        .launch(crate::CONFIG_SERVER_PORT);
}
