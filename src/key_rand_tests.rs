use std::collections::HashMap;

pub fn monobit_test(bits: [u8; 20_000]) -> Result<bool, String> {
    if bits.is_empty() {
        return Err(String::from("Key is empty!"));
    }
    let mut zero = 0;
    let mut one = 0;
    for bit in bits {
        if bit == 0 {
            zero += 1;
        } else if bit == 1 {
            one += 1;
        }
    }
    let result = (9654..10346).contains(&zero) && (9654..10346).contains(&one);
    Ok(result)
}

pub fn max_series_length(bits: [u8; 20_000]) -> Result<bool, String> {
    if bits.is_empty() {
        return Err(String::from("Key is empty!"));
    }
    let mut prev = 2;
    let mut cur_count = 0;

    let mut max_zero_series = 0;
    let mut max_one_series = 0;

    for bit in bits {
        if bit != prev {
            cur_count = 0;
        }
        cur_count += 1;
        if bit == 0 && max_zero_series < cur_count {
            max_zero_series = cur_count;
        }       
        if bit == 1 && max_one_series < cur_count {
            max_one_series = cur_count;
        }
        prev = bit;
    }
    let result = (max_zero_series <= 36) && (max_one_series <= 36);
    Ok(result)
}

pub fn pokker_test(bits: [u8; 20_000]) -> Result<bool, String> {
    if bits.is_empty() {
        return Err(String::from("Key is empty!"));
    }
    let block_size = 4;
    let bits_count = bits.len();
    let blocks_num = bits_count / block_size;

    let mut patterns: HashMap<Vec<u8>, u32> = HashMap::new();

    for i in (0..bits_count).step_by(4) {
        let mut pattern: Vec<u8> = vec![];
        for j in i..(i + block_size) {
            pattern.push(bits[j])
        }
        if patterns.contains_key(&pattern) {
            *patterns.get_mut(&pattern).unwrap() += 1;
        } else {
            patterns.insert(pattern, 1);
        }
    }
    let mut chi_pokker = 0.0;
    for (_, count) in patterns.iter() {
        chi_pokker += (*count as f64).powi(2);
    }
    chi_pokker = (2_f64.powi(block_size as i32) / blocks_num as f64) * chi_pokker - blocks_num as f64;

    let result = (1.03..57.4).contains(&chi_pokker);
    Ok(result)
}

pub fn series_length_test(bits: [u8; 20_000]) -> Result<bool, String> {
    if bits.is_empty() {
        return Err(String::from("Key is empty!"));
    }
    let mut series_zero_length: [u32; 6] = [0; 6];
    let mut series_one_length: [u32; 6] = [0; 6];

    let mut prev = 2;
    let mut cur_count = 0;

    for bit in bits {
        if bit != prev {
            if cur_count >= 6 {
                cur_count = 6;
            }
            if prev == 0 && cur_count != 0 {
                series_zero_length[(cur_count - 1) as usize] += 1;
            } else if cur_count != 0 {
                series_one_length[(cur_count - 1) as usize] += 1;
            }
            cur_count = 0;
        }
        cur_count += 1;
        prev = bit;
    }
    if cur_count != 0 {
        if cur_count > 6 {
            cur_count = 7;
        }
        if prev == 0 && cur_count != 0 {
            series_zero_length[(cur_count - 1) as usize] += 1;
        } else if cur_count != 0 {
            series_one_length[(cur_count - 1) as usize] += 1;
        }
    }
    let one_series = (2265..2733).contains(&series_zero_length[0]) && (2265..2733).contains(&series_one_length[0]);
    let two_series = (1079..1421).contains(&series_zero_length[1]) && (1079..1421).contains(&series_one_length[1]);
    let three_series = (502..748).contains(&series_zero_length[2]) && (502..748).contains(&series_one_length[2]);
    let four_series = (223..402).contains(&series_zero_length[3]) && (223..402).contains(&series_one_length[3]);
    let five_series = (90..223).contains(&series_zero_length[4]) && (90..223).contains(&series_one_length[4]);
    let six_series = (90..223).contains(&series_zero_length[5]) && (90..223).contains(&series_one_length[5]);

    let result = if one_series && two_series && three_series && four_series && five_series && six_series {
        true
    } else {
        false
    };
    Ok(result)
}