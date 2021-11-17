use std::{cmp::Ordering, collections::HashMap};

fn main() {
    // data_source is mutable due to sorting (for median)
    let mut data_source = vec![2, 0, 3, 8, 7, 4, 8, 4, 3, 5, 0, 8, 3, 8, 2, 2, 9, 3, 1, 8, 0, 2, 9, 6, 4, 4, 8, 3, 9, 8];

    println!("Mean of Data Input: {}", calc_mean(&data_source));
    println!("Median of Data Input: {}", retrieve_median(&mut data_source));
    println!("Mode of Data Input: {:?}", retrieve_mode(&data_source));
}

fn calc_mean(data_source: &Vec<i32>) -> f64 {
    let mut sum: f64 = 0.0;
    for data in data_source {
        sum += *data as f64;
    }

    sum / (data_source.len() as f64)
}

fn retrieve_median(data_source: &mut Vec<i32>) -> f64 {
    // for median, data input must be sorted
    data_source.sort_unstable();

    // depending on the length of the vector, either 1 or the mean of 2 values must be returned
    if data_source.len() % 2 != 0 {
        data_source[data_source.len() / 2 + 1] as f64
    } else {
        let first_value = data_source[data_source.len() / 2] as f64;
        let second_value = data_source[data_source.len() / 2 + 1] as f64;
        (first_value + second_value) / 2.0
    }
}

// TODO: multiple numbers may have the same amount
fn retrieve_mode(data_source: &Vec<i32>) -> Vec<&i32> {
    let mut mode_map = HashMap::new();

    for value in data_source {
        let counter = mode_map.entry(value).or_insert(0);
        *counter += 1;
    }

    let mut highest_amount = 0;
    let mut highest_keys = Vec::new();

    for (key, value) in mode_map {
        match value.cmp(&highest_amount) {
            Ordering::Greater => {
                highest_amount = value;
                highest_keys.clear();
                highest_keys.push(key);
            },
            Ordering::Equal => {
                highest_keys.push(key);
            },
            Ordering::Less => (),
        }
    }

    highest_keys
}
