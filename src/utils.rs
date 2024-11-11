use std::{fs, mem};

pub fn generate_random_string(length: usize) -> String {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    thread_rng().sample_iter(&Alphanumeric).take(length).map(char::from).collect()
}

pub fn get_domain_url(url: &str) -> String {
    let indices: Vec<(usize, &str)> = url.match_indices("/").collect();
    url.split_at(indices[2].0).0.to_string()
}

pub fn patch_binary_with_random(binary_path: &str, pattern: &[u8], random_string_size: usize) {
    let mut binary = fatal_unwrap_e!(fs::read(binary_path), "Failed to read target binary for patching {}");
    let pattern = pattern;
    let new_string = generate_random_string(random_string_size);
    // TODO use strings instead of bytes
    let mut matches = Vec::with_capacity(3);
    for (index, window) in binary.windows(pattern.len()).enumerate() {
        if window == pattern {
            matches.push(index);
        }
    }
    if matches.len() == 0 {
        info!("no pater matches found, no need to patch!");
        return;
    }

    let first_match = unsafe { String::from_raw_parts(binary.as_mut_ptr().add(matches[0]), random_string_size, random_string_size) };
    info!("Replacing {} with {}", first_match, new_string);
    mem::forget(first_match);

    for index in matches {
        let mut pattern_str = unsafe { String::from_raw_parts(binary.as_mut_ptr().add(index), random_string_size, random_string_size) };
        pattern_str.replace_range(0..random_string_size, &new_string);
        mem::forget(pattern_str);
    }
    fs::write(binary_path, binary).unwrap();
}
