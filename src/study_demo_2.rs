use std::collections::HashSet;

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
#[allow(dead_code)]
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
// input n k m   n 是数组长度  k 是起始位置  m 出去的位置
// output num the last one
// 做一件事情前把 你要的所有元素都列出来
#[allow(dead_code)]
fn study_demo_5(n: i32, k: i32, m: i32) -> i32 {
    if n == 1 {
        return 0;
    }
    let mut people: Vec<i32> = (0..n).collect();
    let mut current_index = (k - 1) as usize % n as usize;
    while people.len() > 1 {
        current_index = (current_index + m as usize - 1) % people.len();
        people.remove(current_index);
    }
    people[0]
}

#[test]
fn test_study_demo_5() {
    let result = study_demo_5(10, 2, 3);
    println!("{}", result);
}

// pub fn study_demo_6() {
//     let full_vec: Vec<i32> = (0..500).collect();
//     let vec1: Vec<_> = (150..300).collect();
//     let vec2: Vec<_> = (200..400).collect();
//     let mut small_hash_set = HashSet::new();

//     for num in vec1 {
//         small_hash_set.insert(num);
//     }
//     for num in vec2 {
//         small_hash_set.insert(num);
//     }

//     let diff_vec: Vec<i32> = full_vec
//         .into_iter()
//         .filter(|item| !small_hash_set.contains(item))
//         .collect();
//     let result = diff_vec.len();
//     println!("{}", result);
// }
pub fn study_demo_6() {
    let mut input_value_vec = Vec::new();
    for line in std::io::stdin().lines() {
        match line {
            Ok(s) if s.trim().is_empty() => break,
            Ok(s) => input_value_vec.push(s),
            Err(e) => {
                eprintln!("found an error {}", e);
                break;
            }
        }
    }
    let first_line: Vec<i32> = input_value_vec[0]
        .split_whitespace()
        .filter_map(|item| item.parse::<i32>().ok())
        .collect();
    let large_vec_len = first_line[0];
    let _vec_len = first_line[1];
    let large_vec: HashSet<i32> = (0..large_vec_len).collect();
    let mut small_vec = Vec::new();
    for num in input_value_vec.iter().skip(1) {
        let tmp_vec: Vec<i32> = num
            .split_whitespace()
            .filter_map(|item| item.parse::<i32>().ok())
            .collect();
        let start = tmp_vec[0];
        let end = tmp_vec[1];
        let mut vec1: Vec<i32> = (start..end).collect();
        small_vec.append(&mut vec1);
    }
    let diff_vec: Vec<i32> = large_vec
        .into_iter()
        .filter(|item| !small_vec.contains(item))
        .collect();

    println!("{}", diff_vec.len());
}

#[allow(dead_code)]
fn study_demo_7() {
    let rows = 2;
    let cols = 3;
    let mut count = 1;
    let mut grid = vec![vec![0; cols]; rows];
    let mut total_sum = 0;

    // 填充网格并计算总和
    for row in 0..rows {
        for col in 0..cols {
            grid[row][col] = count;
            total_sum += count;
            count += 1;
        }
    }

    // 打印网格
    for row in &grid {
        for &val in row {
            print!("{:4}", val);
        }
        println!();
    }

    println!("总和: {}", total_sum);
}
#[test]
fn test_study_demo_7() {
    study_demo_7();
}
// input   11533
// output  11*33
#[allow(dead_code)]
fn study_demo_8() {
    let str = "11533";
    let output = str.replace("5", "*");
    println!("{}", output);
}
#[test]
fn test_study_demo_8() {
    study_demo_8();
}
// {"elephant","tiger","cat","mouse"}
#[allow(dead_code)]
enum Animal {
    Elephant,
    Tiger,
    Cat,
    Mouse,
}
#[allow(dead_code)]
fn study_demo_9() {
    let enum1: Animal = Animal::Mouse;
    let enum2: Animal = Animal::Elephant;

    match (enum1, enum2) {
        (Animal::Elephant, Animal::Tiger) => println!("win"),
        (Animal::Tiger, Animal::Cat) => println!("win"),
        (Animal::Cat, Animal::Mouse) => println!("win"),
        (Animal::Mouse, Animal::Elephant) => println!("win"),
        _ => println!("tie"),
    }
}
#[test]
fn test_study_demo_9() {
    study_demo_9();
}
// input bobaaa   output 0
// 因为 bob 出现在索引 0
#[allow(dead_code)]
fn study_demo_10(input: &str, pattern: &str) -> Vec<usize> {
    let mut indices = Vec::new();
    let mut start = 0;
    while let Some(i) = input[start..].to_lowercase().find(&pattern.to_lowercase()) {
        let pos = start + i;
        indices.push(pos);
        start = pos + pattern.len();
    }
    indices
}
#[test]
fn test_study_demo_10() {
    let result = study_demo_10("boboboboaaa", "bo");
    println!("{:?}", result);
}

// 56 凯撒加密
// input 1 错位一个  abcdefg
// 输出  bcdefgh
#[allow(dead_code)]
fn study_demo_11(str: String, n: u8) -> String {
    let encrypted: String = str
        .chars()
        .map(|c| {
            let base = if c.is_ascii_lowercase() {
                c as u8 - b'a'
            } else {
                return c;
            };
            let encrypted_char = (base + n) % 26 + b'a';
            encrypted_char as char
        })
        .collect();
    encrypted
}
#[allow(dead_code)]
fn uncover_encrypted(str:String,n:u8) -> String {
    let uncover:String = str.chars().map(|c| {
        let base = if c.is_ascii_lowercase(){
            c as u8 - b'a'
        }else {
            return c;
        };
        let uncover_char = (base - n) % 26 + b'a';
        uncover_char as char
    }).collect();
    uncover
}
#[test]
fn test_study_demo_11() {
    let str = "abcdefg".to_string();
    let n: u8 = 3;
    let result = study_demo_11(str, n);
    println!("{}", result);
}
#[test]
fn test_uncover_encrypted() {
    let str = "defghij".to_string();
    let n: u8 = 3;
    let result = uncover_encrypted(str, n);
    println!("{}", result);
}
// 输入第n 个字符是什么
#[allow(dead_code)]
fn study_demo_12() {
    let str = "123456789101112131415161718".to_string();
    let vec: Vec<_> = str.chars().into_iter().collect();
    println!("{:?}", vec[11])
}
#[test]
fn test_study_demo_12() {
    study_demo_12();
}
// input International Collegiate Programming Contest
// output ICPC
#[allow(dead_code)]
fn study_demo_13() {
    let str = "International Collegiate Programming Contest".to_string();
    let vec_str: Vec<&str> = str.split_whitespace().collect();
    let mut result = String::new();
    for str in vec_str {
        let first_char = str.chars().nth(0);
        if let Some(ch) = first_char {
            result.push(ch);
        }
    }
    println!("{}", result);
}
#[test]
fn test_study_demo_13() {
    study_demo_13();
}

// 将 980364535 转换为 980,364,535
#[allow(dead_code)]
fn study_demo_14(num: &str) -> String {
    // 处理负号情况
    let (sign, num_str) = if let Some(stripped) = num.strip_prefix('-') {
        ("-", stripped)
    } else {
        ("", num)
    };

    // 反转数字字符串以便处理
    let reversed: String = num_str.chars().rev().collect();

    // 分割为3位一组的块
    let chunks: Vec<_> = reversed
        .chars()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect();

    // 用逗号连接块并反转回正常顺序
    let with_commas: String = chunks.join(",").chars().rev().collect();

    // 添加符号
    format!("{}{}", sign, with_commas)
}
#[test]
fn test_study_demo_14() {
    let num = "1123456789";
    println!("{}", study_demo_14(num))
}
// input wxhak  将 1-5 的w 换成b
#[allow(dead_code)]
fn study_demo_15(str: &str,from_str:&str,to_str:&str) -> String {
    let str = str.to_string();
    let result_str = &str[0..5].replace(from_str,to_str);
    result_str.to_string()
}
#[test]
fn test_study_demo_15() {
    let str = "abcdefg";
    let result = study_demo_15(str,"a","ttt");
    println!("{}", result);
}
// 阶乘
#[allow(dead_code)]
fn study_demo_16(n:u32) -> u32 {
    let result = (1..=n).product();
    return result;
}
#[test]
fn test_study_demo_16() {
    println!("{}",study_demo_16(5));
}


