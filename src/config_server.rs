use ini::Ini;
use ramhorns::{Content, Template};
use std::{env, process::Command, thread, time::Duration};
use web_server::{redirect, HttpCode, Request, Response};

mod save_data;

macro_rules! exe_dir {
    () => {
        env::current_exe()
            .expect("Cannot find current exe")
            .parent()
            .unwrap()
            .to_path_buf()
    };
}

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
        .get(
            "/",
            Box::new(|_, mut resp: Response| {
                let mut path = exe_dir!();
                path.push("./static/index.html");
                let path = path.to_str().unwrap();

                let tpl = if let Ok(template) = Template::new(path) {
                    template
                } else {
                    resp.set_http_code(HttpCode::_500)
                        .set_body("HTML template is incorrect");
                    return resp;
                };

                let t = tpl.render(&SetupContent::from_ini("add_record_config.ini"));

                "asd".into()
            }),
        )
        .get("/index.html", redirect("/"))
        .post("/save", Box::new(save_data::save))
        .get(
            "/close",
            Box::new(move |_, _| {
                thread::spawn(move || {
                    thread::sleep(Duration::from_millis(600));
                    // Because server must respond to client
                    on_close();
                });
                "ok".into()
            }),
        )
        .not_found(Box::new(|req: Request, _| {
            let mut path = exe_dir!();
            path.push(format!("./static{}", req.get_path()));

            path.as_path().into()
        }))
        .launch(crate::CONFIG_SERVER_PORT);
}

#[derive(Content, Clone)]
struct SetupContent {
    output_filename: String,
    fields: Vec<String>,
}

impl SetupContent {
    fn from_ini(path: &str) -> Result<SetupContent, ini::ini::Error> {
        let mut abs_path = exe_dir!();
        abs_path.push(path);
        let abs_path = abs_path.to_str().unwrap();

        let data = Ini::load_from_file(abs_path)?;
        let data = data.section::<String>(None).expect("Zły format pliku .ini");

        let output_filename = data
            .get("output_file_name")
            .unwrap_or("add_record_config.ini")
            .to_string();

        let fields = data.get("fields").unwrap().to_string();
        let fields: Vec<_> = fields.split(',').map(|item| item.to_string()).collect();

        Ok(Self {
            output_filename,
            fields,
        })
    }
}
