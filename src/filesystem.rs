pub fn normalize_path(path: &str) -> String {
    if cfg!(windows) {
        path.to_owned()
    } else {
        path.replace(":", "")
    }
}
