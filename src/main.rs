use std::cmp::Ordering;

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
fn kth_smallest(mut arr: Vec<i32>, k: usize) -> i32 {
    // Sort the given array
    arr.sort();

    // Return K'th element in the sorted array
    arr[k - 1]
}
// Problem 8
    struct Node {
    data: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(item: i32) -> Self {
        Node { data: item, left: None, right: None }
    }
}

struct BinaryTree {
    root: Option<Box<Node>>,
}

impl BinaryTree {
    fn max_depth(&self, node: &Option<Box<Node>>) -> i32 {
        match node {
            Some(n) => {
                let l_depth = self.max_depth(&n.left);
                let r_depth = self.max_depth(&n.right);
                1 + l_depth.max(r_depth)
            }
            None => 0,
        }
    }
}

// Problem 8
fn merge_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut merged = Vec::with_capacity(arr1.len() + arr2.len());
    let mut i = 0;
    let mut j = 0;

    while i < arr1.len() && j < arr2.len() {
        match arr1[i].cmp(&arr2[j]) {
            Ordering::Less => {
                merged.push(arr1[i]);
                i += 1;
            }
            Ordering::Equal => {
                merged.push(arr1[i]);
                merged.push(arr2[j]);
                i += 1;
                j += 1;
            }
            Ordering::Greater => {
                merged.push(arr2[j]);
                j += 1;
            }
        }
    }

    // Add remaining elements from arr1
    while i < arr1.len() {
        merged.push(arr1[i]);
        i += 1;
    }

    // Add remaining elements from arr2
    while j < arr2.len() {
        merged.push(arr2[j]);
        j += 1;
    }

    merged
}
fn max_sub_array_sum(a: &[i32]) -> i32 {
    let mut max_so_far = i32::MIN;
    let mut max_ending_here = 0;

    for &num in a {
        max_ending_here = max_ending_here + num;
        if max_so_far < max_ending_here {
            max_so_far = max_ending_here;
        }
        if max_ending_here < 0 {
            max_ending_here = 0;
        }
    }
    
    max_so_far
}
fn main() {
    // Input string
    //01
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
    //07
    let qus7_arr = vec![4, 2, 6, 1, 3, 5]; // Example array
    let qus7_k = 3; // Example value of K
    println!("07 - The kth smallest element in the given array is {}", kth_smallest(qus7_arr, qus7_k));
    // 08
    let mut tree = BinaryTree { root: None };

    tree.root = Some(Box::new(Node::new(1)));
    if let Some(ref mut root) = tree.root {
        root.left = Some(Box::new(Node::new(2)));
        root.right = Some(Box::new(Node::new(3)));
        if let Some(ref mut left) = root.left {
            left.left = Some(Box::new(Node::new(4)));
            left.right = Some(Box::new(Node::new(5)));
        }
    }
    println!("08 - Height of tree is {}", tree.max_depth(&tree.root));

    // 09
    let que9_str = "BLOCKCHAIN";
    let mut que9_nstr = String::new();


    for ch in que9_str.chars() {
        que9_nstr = ch.to_string() + &que9_nstr;
    }
    println!("09 - Original word: {} Reversed word: {}", que9_str, que9_nstr);
    // 10
    let prime_number2 = 211;
    println!("10 - Is {} prime ? Answer: {}", prime_number2, is_prime(prime_number2));

    // 11
    let arr1 = vec![1, 3, 5, 7];
    let arr2 = vec![2, 4, 6, 8];

    let merged = merge_arrays(&arr1, &arr2);

    print!("11 - Array after merging: ");
    for num in merged {
        print!("{} ", num);
    }
    println!();
    //12
    let max_subarray = [-2, -3, 4, -1, -2, 1, 5, -3];
    println!("12 - Maximum contiguous sum in array A is {}", max_sub_array_sum(&max_subarray));
}
