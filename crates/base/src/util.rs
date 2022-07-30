use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn rand_string(len: usize) -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect();

    rand_string
}

#[cfg(test)]
mod util_tests {
    use super::rand_string;

    #[test]
    fn test_rand_string() {
        let password = rand_string(10);

        assert_eq!(10, password.len());
    }
}
