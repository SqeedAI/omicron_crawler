use crate::errors::IoError::FileError;
use crate::errors::IoResult;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;

pub fn generate_random_string(length: usize) -> String {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    thread_rng().sample_iter(&Alphanumeric).take(length).map(char::from).collect()
}

pub fn load_file_as_str(path: &str) -> IoResult<String> {
    let mut file = match fs::File::open(path) {
        Ok(file) => file,
        Err(_) => {
            return Err(FileError(format!("Failed to open file {}", path)));
        }
    };

    let mut buff = String::new();
    if let Err(error_code) = file.read_to_string(&mut buff) {
        return Err(FileError(format!("Failed to read file {}", error_code)));
    }
    Ok(buff)
}

pub fn save_to_file(bytes: &[u8], path: &str) -> IoResult<()> {
    let path_sys = Path::new(path);

    if let Some(parent) = path_sys.parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            return Err(FileError(format!("Failed to create directory: {}", e)));
        }
    }

    let mut file = match fs::File::create(path_sys) {
        Ok(file) => file,
        Err(e) => {
            return Err(FileError(format!("Failed to open file {}", e)));
        }
    };
    if let Err(e) = file.write_all(bytes) {
        return Err(FileError(format!("Failed to write file {}", e)));
    }
    info!("Bytes written to to {}", path);
    Ok(())
}
