#[cfg(test)]
mod test {
    use rand::Rng;
    use crate::key_rand_tests::*;

    #[test]
    fn monobyte_tests() {
        let mut key: [u8; 20_000] =[0; 20_000];
        for i in 0..20_000 {
            key[i] = rand::thread_rng().gen_range(0..2);
        }
        assert!(monobyte_test(key).unwrap());
    }

    #[test]
    fn max_series_length_tests() {
        let mut key: [u8; 20_000] =[0; 20_000];
        for i in 0..20_000 {
            key[i] = rand::thread_rng().gen_range(0..2);
        }
        assert!(max_series_length(key).unwrap());
    }

    #[test]
    fn pokker_tests() {
        let mut key: [u8; 20_000] =[0; 20_000];
        for i in 0..20_000 {
            key[i] = rand::thread_rng().gen_range(0..2);
        }
        assert!(pokker_test(key).unwrap());
    }

    #[test]
    fn series_length_tests() {
        let mut key: [u8; 20_000] =[0; 20_000];
        for i in 0..20_000 {
            key[i] = rand::thread_rng().gen_range(0..2);
        }
        assert!(series_length_test(key).unwrap());
    }
}