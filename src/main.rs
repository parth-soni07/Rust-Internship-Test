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
    let prime_number1 = 114;
    println!("04 - Is {} prime ? Answer: {}", prime_number1, is_prime(prime_number1));
}
