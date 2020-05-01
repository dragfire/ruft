use rand::Rng;

/// Generate a random number between 150 and 300
/// 
/// It is used in hearbeat and election timeout
pub fn random_timeout() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(150, 301)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_timeout() {
        assert!(random_timeout() >= 150 && random_timeout() <= 300);
    }
}
