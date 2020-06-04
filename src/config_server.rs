use ini::Ini;
use serde::Serialize;
use std::{env, fs::read_to_string, process::Command, thread, time::Duration};
use tinytemplate::TinyTemplate;
use web_server::{utils::redirect, HttpCode, Request, Response};

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
        .any(
            "/",
            Box::new(move |_, mut resp: Response| {
                let index = get_index_html();
                if let Ok(index) = &index {
                    index.as_str().into()
                } else {
                    resp.set_http_code(HttpCode::_500)
                        .set_body("HTML template is incorrect");
                    resp
                }
            }),
        )
        .any("/index.html", redirect("/"))
        .post("/save", Box::new(save_data::save))
        .any(
            "/close",
            Box::new(move |_, _| {
                thread::spawn(move || {
                    thread::sleep(Duration::from_millis(600));
                    // Because server must respond to client
                    on_close();
                });
                "closed".into()
            }),
        )
        .not_found(Box::new(|req: Request, _| {
            let mut path = exe_dir!();
            path.push(format!("./static{}", req.get_path()));

            path.as_path().into()
        }))
        .launch(crate::CONFIG_SERVER_PORT);
}

fn get_index_html() -> Result<String, tinytemplate::error::Error> {
    let mut path = exe_dir!();
    path.push("./static/index.html");
    let path = path.to_str().unwrap();
    let index = read_to_string(path).expect("Cannot find index.html");

    let mut tpl = TinyTemplate::new();
    tpl.add_template("index", index.as_str())?;

    let variables = SetupContent::from_ini("add_record_config.ini");

    let html = tpl.render("index", &variables)?;

    Ok(html)
}

#[derive(Serialize)]
struct SetupContent {
    output_filename: Option<String>,
    fields: Option<Vec<String>>,
    error: Option<String>,
}

impl SetupContent {
    fn from_ini(path: &str) -> Self {
        let vars = Self::from_ini_raw(path);

        if let Ok(vars) = vars {
            vars
        } else {
            Self {
                error: Some("Plik ini jest uszkodzony".into()),
                fields: None,
                output_filename: None,
            }
        }
    }

    fn from_ini_raw(path: &str) -> Result<Self, ini::ini::Error> {
        let mut abs_path = exe_dir!();
        abs_path.push(path);
        let abs_path = abs_path.to_str().unwrap();

        let data = Ini::load_from_file(abs_path)?;
        let data = data.section::<String>(None).expect("Zły format pliku .ini");

        let output_filename = Some(
            data.get("output_file_name")
                .unwrap_or("add_record_config.ini")
                .to_string(),
        );

        let fields = data.get("fields").unwrap().to_string();
        let fields: Vec<_> = fields.split(',').map(|item| item.to_string()).collect();

        Ok(Self {
            output_filename,
            fields: Some(fields),
            error: None,
        })
    }
}
