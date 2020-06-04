use std::env;

pub fn file<'a>(name: &'a str) -> String {
    let mut exe_dir = env::current_exe()
        .expect("Cannot find current exe")
        .parent()
        .unwrap()
        .to_path_buf();

    exe_dir.push(name);
    let path = exe_dir.to_str().unwrap();
    String::from(path)
}
