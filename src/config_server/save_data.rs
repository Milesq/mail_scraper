use web_server::{Request, Response};

pub fn save(req: Request, _: Response) -> Response {
    println!("{:#?}", req);
    "".into()
}
