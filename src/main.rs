fn is_palindrome(s: &str) -> bool {
    let rev: String = s.chars().rev().collect();
    s == rev
}
fn first_occurence(n: usize, key: i32, v: &[i32]) -> i32 {
    let mut res = -1;
    for i in 0..n {
        if v[i] == key {
            res = i as i32;
            break;
        }
    }
    res
}
fn min_length_word(input: &str) -> &str {
    let input = input.trim();

    let mut min_length = input.len();
    let mut min_start_index = 0;

    let mut si = 0;
    let mut ei = 0;

    while ei <= input.len() {
        if ei < input.len() && !input[ei..].starts_with(' ') {
            ei += 1;
        } else {
            let curr_length = ei - si;

            if curr_length < min_length {
                min_length = curr_length;
                min_start_index = si;
            }
            si = ei + 1;
            ei += 1;
        }
    }

    &input[min_start_index..min_start_index + min_length]
}

fn is_prime(n: u32) -> bool {
    // Corner case
    if n <= 1 {
        return false;
    }

    // Check from 2 to n-1
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }

    true
}
fn find_median(mut a: Vec<i32>) -> f64 {
    let n = a.len();
    // check for even case
    if n % 2 != 0 {
        return a[n / 2] as f64;
    }
    (a[(n - 1) / 2] + a[n / 2]) as f64 / 2.0
}
fn longest_common_prefix(a: &[String]) -> String {
    let size = a.len();
    // if size is 0, return empty string
    if size == 0 {
        return String::new();
    }
    if size == 1 {
        return a[0].clone();
    }
    // sort the array of strings
    let mut sorted = a.to_vec();
    sorted.sort();

    // find the minimum length from first and last string
    let end = std::cmp::min(sorted[0].len(), sorted[size - 1].len());

    // find the common prefix between the first and last string
    let mut i = 0;
    while i < end && sorted[0].chars().nth(i) == sorted[size - 1].chars().nth(i) {
        i += 1;
    }

    let pre = sorted[0][..i].to_string();
    pre
}
fn main() {
    // Input string
    let mut str = String::from("geeg");

    // Convert the string to lowercase
    str.make_ascii_lowercase();
    let a = is_palindrome(&str);
    println!("01 - Is the string {} palindrome? Answer: {}",str, a);

    //02
    let n = 7;
    let key = 13;
    let v = [3, 4, 13, 13, 13, 20, 40];
    println!("02 - The first occurence of {} in the given array is at index {}", key,first_occurence(n, key, &v));
    //03
    let given_string = "Find the maximum subarray sum Rust";
    let min_word = min_length_word(given_string);
    println!("03 - Minimum length word: {}", min_word);
    //04
    let prime_number1 = 114;
    println!("04 - Is {} prime ? Answer: {}", prime_number1, is_prime(prime_number1));
    //05
    let sorted_array = vec![3, 1, 5, 4, 2]; // Example array
    println!("05 - Median of the array is {}", find_median(sorted_array));
    //06
    let string_set = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()]; // Example array
    println!("06 - The Longest common prefix of the given string is : {}", longest_common_prefix(&string_set));
}
