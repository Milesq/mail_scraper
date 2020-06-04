use std::{fs::read_to_string, env};
use web_server::{Request, Response};

pub fn save(req: Request, _: Response) -> Response {
    println!("{:#?}", req);

    let mut path = env::current_exe()
            .expect("Cannot find current exe")
            .parent()
            .unwrap()
            .to_path_buf();

    path.push("./static/saved.html");
    let path = path.to_str().unwrap();
    let file = read_to_string(path);

    file.unwrap().into()
}
