
use std::ascii::AsciiExt;

pub fn url_fmt(method: &str, path: &str) -> String {
    println!("{:?}", (path, method));
    return method.to_ascii_lowercase() + "_" + path;
}
