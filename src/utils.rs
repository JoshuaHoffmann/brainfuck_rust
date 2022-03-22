pub fn new_extension(name: &str, extension: &str) -> Result<String, &'static str> {
    let mut new_name = String::new();

    let chopped_path:Vec<&str> = name.split('/').collect();
    let filename = *match chopped_path.last() {
        Some(s) => s,
        None => return Err("Empty or malformed path."),
    };

    let filename_body = match filename.split('.').last() {
        Some(a) => a,
        None => return Err("Empty or malformed filename"),
    };

    new_name.push_str(filename_body);
    new_name.push_str(extension);

    return Ok(new_name);
}