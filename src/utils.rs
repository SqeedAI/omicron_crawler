// Function to generate a random string (as defined in the previous answer)
pub fn generate_random_string(length: usize) -> String {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    thread_rng().sample_iter(&Alphanumeric).take(length).map(char::from).collect()
}

pub fn get_domain_url(url: &str) -> String {
    let indices: Vec<(usize, &str)> = url.match_indices("/").collect();
    url.split_at(indices[2].0).0.to_string()
}
