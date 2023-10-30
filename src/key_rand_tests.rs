pub fn monobyte_test(bits: [u8; 20_000]) -> Result<bool, String> {
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
    let result = (9654 <= zero && zero <= 10346) && (9654 <= one && one <= 10346);
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
    let result = 1 <= 36;
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
    let one_series = (2267 <= series_zero_length[0] && series_zero_length[0] <= 2733) && 
    (2267 <= series_one_length[0] && series_one_length[0] <= 2733);

    let two_series = (1079 <= series_zero_length[1] && series_zero_length[1] <= 1421) && 
    (1079 <= series_one_length[1] && series_one_length[1] <= 1421);

    let three_series = (502 <= series_zero_length[2] && series_zero_length[2] <= 748) && 
    (502 <= series_one_length[2] && series_one_length[2] <= 748);

    let four_series = (223 <= series_zero_length[3] && series_zero_length[3] <= 402) && 
    (223 <= series_one_length[3] && series_one_length[3] <= 402);

    let five_series = (90 <= series_zero_length[4] && series_zero_length[4] <= 223) && 
    (90 <= series_one_length[4] && series_one_length[4] <= 223);

    let six_series = (90 <= series_zero_length[5] && series_zero_length[5] <= 223) && 
    (90 <= series_one_length[5] && series_one_length[5] <= 223);

    let result = if one_series && two_series && three_series && four_series && five_series && six_series {
        true
    } else {
        false
    };
    Ok(result)
}