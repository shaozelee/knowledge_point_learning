// input 8 15 3 42 7 0
// output 7 42 3 15 8
pub fn study_demo_1() {
    let mut input_buf = String::new();
    std::io::stdin()
        .read_line(&mut input_buf)
        .expect("not valid number");

    let nums: Vec<i32> = input_buf
        .trim()
        .split_whitespace()
        .filter_map(|x| x.parse::<i32>().ok())
        .filter(|&x| x > 0)
        .rev()
        .collect();
    let mut result = String::new();
    for num in nums {
        result.push_str(&num.to_string());
        result.push(' ');
    }
    println!("{}", result);
}
// input 6  4 4 2 1 3 5
// output   0 0 0 0 2 5
pub fn study_demo_2() {
    let mut new_lines = Vec::new();
    for line in std::io::stdin().lines() {
        match line {
            Ok(s) if s.trim().is_empty() => break,
            Ok(s) => new_lines.push(s),
            Err(e) => {
                eprintln!("{}", e);
                break;
            }
        }
    }
    let new_vec: Vec<i32> = new_lines[1]
        .split_whitespace()
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();
    let mut output_vec: Vec<i32> = Vec::new();

    for i in 0..new_vec.len() {
        let mut count = 0;
        for j in 0..i {
            if new_vec[j] < new_vec[i] {
                count += 1;
            }
        }
        output_vec.push(count);
    }
    let mut result = String::new();
    for num in output_vec {
        result.push_str(&num.to_string());
        result.push(' ');
    }
    println!("{}", result);
}

// input  1,2,3,4,5
// output max_diff = 5 - 1 = 4 avg = (1+2+3+4+5)/5
#[allow(dead_code)]
fn study_demo_3(nums: Vec<i32>) -> String {
    let max_val = nums.iter().max().unwrap();
    let min_val = nums.iter().min().unwrap();
    let sum_val: f64 = nums.iter().map(|&n| n as f64).sum();

    let avg_val = sum_val / nums.len() as f64;
    let str = format!("{} {}", max_val - min_val, avg_val);
    str
}
#[test]
fn test_study_demo_3() {
    let nums = vec![1, 2, 3, 44, 5];
    let result = study_demo_3(nums);
    println!("{}", result);
}
// input 1 2 3 10 11   1 出现4次
fn study_demo_4(nums: Vec<i32>, n: i32) -> i32 {
    let mut str_result = String::new();
    for num in nums {
        str_result.push_str(&num.to_string());
    }
    let output_val: usize = str_result
        .chars()
        .filter_map(|x| x.to_digit(10))
        .filter(|&x| x == n as u32)
        .count();
    output_val as i32
}
#[test]
fn test_study_demo_4() {
    let nums = vec![1, 2, 3, 11, 12];
    let n = 1;
    assert_eq!(study_demo_4(nums, n), 4);
}
// 约瑟夫环
