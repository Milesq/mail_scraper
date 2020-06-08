use ini::Ini;
use std::path::Path;
use web_server::{decoders::x_www_form_urlencoded, Request, Response};

static SAVEDHTML: &str = "
  <h2>Dane zapisane</h2>

  <script>
    setTimeout(() => {
      location.pathname = '/';
    }, 300);
  </script>
";

pub fn save(req: Request, _: Response) -> Response {
    let mut body = req.get_body();
    body = urlencoding::decode(&body).unwrap();
    let body = x_www_form_urlencoded(&body);

    let mut ini = Ini::new();
    ini.with_section::<String>(None)
        .set(
            "output_filename",
            body.get("output_filename").unwrap().as_str(),
        )
        .set("fields", body.get("fields").unwrap().as_str());

    ini.write_to_file(crate::file("add_record_config.ini"))
        .expect("Nie można zapisać pliku INI");

    Path::new(SAVEDHTML).into()
}
