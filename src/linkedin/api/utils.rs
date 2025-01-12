use std::io::Read;

pub fn load_cookies() -> Option<String> {
    let mut file = match std::fs::File::open("cookies.dat") {
        Ok(file) => file,
        Err(_) => {
            info!("Failed to open cookies file");
            return None;
        }
    };
    let mut cookies = String::new();
    if let Err(error_code) = file.read_to_string(&mut cookies) {
        error!("Failed to read cookies file {}", error_code);
        return None;
    }
    Some(cookies)
}
