use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn random_string(length: usize) -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();

    return rand_string;
}
