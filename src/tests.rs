#[cfg(test)]
mod test {
    use rand::Rng;
    use crate::key_rand_tests::*;

    #[test]
    fn monobyte_tests() {
        let mut key: [u8; 2_500] =[0; 2_500];
        for i in 0..key.len() {
            key[i] = rand::thread_rng().gen_range(0..u8::MAX);
        }
        assert!(monobit_test(key).unwrap());
    }

    #[test]
    fn max_series_length_tests() {
        let mut key: [u8; 2_500] =[0; 2_500];
        for i in 0..key.len() {
            key[i] = rand::thread_rng().gen_range(0..u8::MAX);
        }
        assert!(max_series_length(key).unwrap());
    }

    #[test]
    fn pokker_tests() {
        let mut key: [u8; 2_500] =[0; 2_500];
        for i in 0..key.len() {
            key[i] = rand::thread_rng().gen_range(0..u8::MAX);
        }
        assert!(pokker_test(key).unwrap());
    }

    #[test]
    fn series_length_tests() {
        let mut key: [u8; 2_500] =[0; 2_500];
        for i in 0..key.len() {
            key[i] = rand::thread_rng().gen_range(0..u8::MAX);
        }
        assert!(series_length_test(key).unwrap());
    }
}