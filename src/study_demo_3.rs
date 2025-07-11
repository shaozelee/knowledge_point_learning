pub fn study_demo_1() {
    let mut input_vec = Vec::new();
    for line in std::io::stdin().lock().lines() {
        match line {
            Ok(input) if input.is_empty() => break,
            Ok(input) => input_vec.push(input),
            Err(error) => println!("error: {}", error),
        }
    }
    let mut output_str = HashMap::new();
    // input xm  100 90 80
    //       ah  90 80 100
    // output         成绩最大的
    use std::collections::HashMap;
    use std::io::BufRead;
    for score_line in &input_vec {
        let line_for_parse: Vec<&str> = score_line.split_whitespace().collect();
        output_str
            .entry(line_for_parse[0])
            .or_insert(Vec::new())
            .push(line_for_parse[1..].join(" "));
    }
    let mut max_total = i32::MIN;
    let mut max_name = String::new();
    let mut max_score_line = String::new();
    for (key_name, value_scores) in output_str {
        for score_line in &value_scores {
            let current_total: i32 = score_line
                .split_whitespace()
                .filter_map(|x| x.parse::<i32>().ok())
                .sum();

            if current_total > max_total {
                max_total = current_total;
                max_name = String::from(key_name);
                max_score_line = String::from(score_line);
            }
        }
    }
    println!("{} {}", max_name, max_score_line);
}

struct Point {
    x: i32,
    y: i32,
}
impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }
}
fn point_distance(a: &Point, b: &Point) -> i32 {
    // 计算 x 坐标差的绝对值
    let dx = (a.x - b.x).abs();
    // 计算 y 坐标差的绝对值
    let dy = (a.y - b.y).abs();
    // 返回曼哈顿距离
    dx + dy
}
pub fn study_demo_3() {
    let point1 = Point::new(0, 0);
    let point2 = Point::new(2, 3);
    let distance = point_distance(&point1, &point2);
    println!("{}", distance);
}
#[allow(dead_code)]
fn is_excellent(score_1: f64, score_2: f64) -> bool {
    let score = score_1 * 0.7 + score_2 * 0.3;
    if score > 60.0 { true } else { false }
}
#[test]
fn test_is_excellent() {
    assert!(is_excellent(70.0, 70.0));
}
// 序列操作
pub fn study_demo_4() {
    let mut vec: Vec<i32> = Vec::new();
    vec.push(0);
    vec.push(1);
    vec.push(5);
    vec.push(8);
    vec.push(10);
    vec.push(11);
    vec.pop();
    vec.get(2).unwrap_or(&0);
    vec.sort_by(|a, b| a.cmp(b));
    println!("{:?}", vec);
    vec.sort_by(|a, b| b.cmp(a));
    println!("{:?}", vec);
    let len = vec.len();
    println!("{:?}", len);
}

fn count_peak_point(vec: &Vec<i32>) -> i32 {
    let n = vec.len();
    if n == 0 {
        return 0;
    };
    if n == 1 {
        return 1;
    };

    let mut count = 0;
    // 头部计算
    if vec[0] > vec[1] {
        count += 1;
    }
    // 中间计算
    // for i in 1..vec.len()-1 {
    //     if vec[i] > vec[i-1] && vec[i] > vec[i+1] {
    //         count += 1;
    //     }
    // }
    count += vec
        .windows(3)
        .filter(|w| w[1] > w[0] && w[1] > w[2])
        .count() as i32;

    // 末尾计算
    if vec[vec.len() - 1] > vec[vec.len() - 2] {
        count += 1;
    }
    count
}

#[allow(dead_code)]
fn count_peak_point_2(vec: &Vec<i32>) -> i32 {
    (0..vec.len()).fold(0, |count, i| {
        count
            + match i {
                0 if vec[i] > vec[i + 1] => 1,                            // 首元素
                i if i == vec.len() - 1 && vec[i] > vec[i - 1] => 1,      // 尾元素
                i => (vec[i] > vec[i - 1] && vec[i] > vec[i + 1]) as i32, // 中间元素
            }
    })
}
#[test]
fn test_count_peak_point() {
    let vec = vec![0, 1, 2, 1, 4, 5, 4, 7, 8, 9];
    assert_eq!(count_peak_point(&vec), 3);
}
// how long takes
// input [150,180,170],[35,32,33,34],2,4  [150,180,170],[35->150<-32->180<-33->170<-34] 634
fn how_long_seconds_need(spend: Vec<i32>, dwell: Vec<i32>, start: usize, end: usize) -> i32 {
    dwell.iter().take(end).skip(start - 1).sum::<i32>()
        + spend.iter().take(end - 1).skip(start - 1).sum::<i32>()
}
#[test]
fn test_how_long_seconds_need() {
    let spend_seconds = vec![150, 180, 170];
    let dwell_seconds = vec![35, 32, 33, 34];
    let start = 2;
    let end = 4;
    println!(
        "{}",
        how_long_seconds_need(spend_seconds, dwell_seconds, start, end)
    );
}

// input [1,2,3]
// ouput "1,2,3"
#[allow(dead_code)]
fn vec_to_string(vec: Vec<i32>) -> String {
    vec.iter()
        .map(|x| format!("{}", x))
        .collect::<Vec<String>>()
        .join(",")
}
#[test]
fn test_vec_to_string() {
    println!("{}", vec_to_string(vec![1, 2, 3, 4, 5]));
}
// input [2,1,4,5,2,3],3  走三步  每走一步 看脚下的数字，然后根据数字再走 算出最终在哪里
#[allow(dead_code)]
fn the_end_numbers(vec: Vec<i32>, step: i32) -> i32 {
    if vec.is_empty() {
        return 0;
    }
    let len = vec.len();
    let mut current_index = 0;
    for _ in 0..step {
        let move_value = vec[current_index];
        current_index = (current_index as i64 + move_value as i64).rem_euclid(len as i64) as usize;
    }
    vec[current_index]
}
#[test]
fn test_the_end_numbers() {
    let vec = vec![1, 2, 3, 4, 5];
    assert_eq!(the_end_numbers(vec.clone(), 2), 4);
    assert_eq!(the_end_numbers(vec.clone(), 3), 3);
}
#[allow(dead_code)]
fn vec_multiplication_vec(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    nums1
        .into_iter()
        .zip(nums2.into_iter())
        .fold(0, |acc, (num1, num2)| acc + (num1 * num2))
}
#[test]
fn test_vec_multiplication_vec() {
    let vec1 = vec![1, 2, 3, 4, 5];
    let vec2 = vec![6, 7, 8, 9];
    println!("{}", vec_multiplication_vec(vec1, vec2));
}
// stack
#[allow(dead_code)]
fn study_stack_vec() {
    let mut stack = vec![1, 2, 3, 4, 5];
    stack.push(6);
    stack.push(7);
    stack.push(8);
    stack.pop();
    if let Some(num) = stack.get(stack.len() - 1) {
        println!("{}", num);
    }
    println!("{}", stack.len());
    println!("{:?}", stack);
}
#[test]
fn test_study_stack_vec() {
    study_stack_vec();
}
// input a[x(y)z]   括号合法
fn is_bra_licit(str: String) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for ch in str.chars() {
        match ch {
            '[' | '{' | '(' => {
                stack.push(ch);
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            _ => (),
        }
    }
    stack.is_empty()
}
#[test]
fn test_is_bra_licit() {
    let str = "abcd(){{(())}}efg".to_string();
    assert_eq!(is_bra_licit(str), true);
}
fn is_good_string(s: &str) -> bool {
    let mut stack = 0;
    for c in s.chars() {
        match c {
            'a' => stack += 1,
            'b' => {
                if stack == 0 {
                    return false;
                }
                stack -= 1;
            }
            _ => return false, // 非 'a'/'b' 字符直接返回 false
        }
    }
    stack == 0
}
#[test]
fn test_is_good_string() {
    assert!(is_good_string("ab")); // true
    assert!(is_good_string("aabb")); // true
    assert!(is_good_string("aababb")); // true
    assert!(!is_good_string("aab")); // false
    assert!(!is_good_string("ba")); // false
    assert!(!is_good_string("abbb")); // false
    assert!(!is_good_string("abc")); // false (含非法字符)
}
// input o O   2个小的 oo  变为 一个O   2个大的OO  消失
// oOoOoOoOoOoO

// fn bubbles_game(str: String) -> String {
//     let mut stack: Vec<char> = Vec::new();
//     for ch in str.chars() {
//         // 栈中没有值就插入
//         if (ch == 'o' || ch == 'O') && stack.is_empty() {
//             stack.push(ch);
//         } else {
//             // 对最后一个值和ch 判断
//             if ch == 'o' && &ch == stack.last().unwrap() {
//                 stack.pop();

//                 if let Some(last) = stack.last() {
//                     if 'O' == *last {
//                         stack.pop();
//                     } else {
//                         stack.push('O');
//                     }
//                 }
//             } else if ch == 'O' && &ch == stack.last().unwrap() {
//                 stack.pop();
//             } else {
//                 stack.push(ch);
//             }
//         }
//     }

//     let result = stack.into_iter().collect::<String>();
//     return result;
// }

fn bubbles_game(str: String) -> String {
    let mut stack: Vec<char> = Vec::new();
    for ch in str.chars() {
        match ch {
            'O' => {
                if let Some(last) = stack.last() {
                    if *last == 'O' {
                        stack.pop();
                        continue;
                    }
                }
                stack.push('O');
            }
            'o' => {
                if let Some(last) = stack.last() {
                    if *last == 'o' {
                        stack.pop(); // 消除两个 'o'
                        // 检查新栈顶并决定是否进一步消除或压入 'O'
                        if let Some(next) = stack.last() {
                            if *next == 'O' {
                                stack.pop();
                            } else {
                                stack.push('O');
                            }
                        } else {
                            stack.push('O');
                        }
                        continue;
                    }
                }
                stack.push('o');
            }
            _ => stack.push(ch), // 非 o/O 字符直接入栈
        }
    }
    stack.into_iter().collect()
}
#[test]
fn test_bubbles_game() {
    let str = "ooOOoooO".to_string();
    assert_eq!(bubbles_game(str), "oO");
}
// 模拟计算器 支持 + - *  和 ()
// 3 * ( 3 + 1 )  -> 12
// 优先级 （） *  +-
// 表达式求值
fn solve_express(input_str: String) -> i32 {
    let mut ops: Vec<char> = Vec::new(); // 运算符栈
    let mut vals: Vec<i32> = Vec::new(); // 操作数栈
    let mut current_number = 0; // 跟踪当前构建的数字
    let mut building_number = false; // 标记是否正在构建多位数

    // 定义一个内部函数来应用运算符
    fn apply_top(ops: &mut Vec<char>, vals: &mut Vec<i32>) -> bool {
        if let Some(op) = ops.pop() {
            if op == '+' || op == '-' || op == '*' {
                if vals.len() < 2 {
                    // 操作数不够，放回运算符
                    ops.push(op);
                    return false;
                }
                let b = vals.pop().unwrap();
                let a = vals.pop().unwrap();
                let res = match op {
                    '+' => a + b,
                    '-' => a - b,
                    '*' => a * b,
                    _ => unreachable!(), // 不会出现其他情况
                };
                vals.push(res);
                return true;
            } else {
                // 如果是'('，放回去
                ops.push(op);
                return false;
            }
        }
        false
    }

    // 处理每个字符
    for ch in input_str.chars().filter(|c| !c.is_whitespace()) {
        match ch {
            '(' | ')' | '+' | '-' | '*' => {
                // 完成当前数字的构建（如果有）
                if building_number {
                    vals.push(current_number);
                    current_number = 0;
                    building_number = false;
                }

                // 处理括号和运算符
                match ch {
                    '(' => ops.push(ch),
                    ')' => {
                        // 计算括号内的表达式
                        while let Some(&top_op) = ops.last() {
                            if top_op == '(' {
                                break;
                            }
                            if !apply_top(&mut ops, &mut vals) {
                                break;
                            }
                        }
                        // 弹出左括号
                        if let Some('(') = ops.pop() {
                            // 成功弹出
                        }
                    }
                    op @ ('+' | '-' | '*') => {
                        // 确保优先级顺序：先乘除后加减
                        while let Some(&top_op) = ops.last() {
                            if top_op == '(' {
                                break;
                            }
                            // 当前运算符是+或-，且栈顶是*时，需要先计算栈顶的*
                            if (op == '+' || op == '-') && top_op == '*' {
                                if !apply_top(&mut ops, &mut vals) {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                        ops.push(op);
                    }
                    _ => unreachable!(),
                }
            }
            '0'..='9' => {
                // 构建多位数
                building_number = true;
                current_number = current_number * 10 + (ch as i32 - '0' as i32);
            }
            _ => panic!("Invalid character: {}", ch),
        }
    }

    // 处理表达式末尾的数字
    if building_number {
        vals.push(current_number);
    }

    // 处理栈中剩余运算符
    while !ops.is_empty() {
        apply_top(&mut ops, &mut vals);
    }

    vals.pop().unwrap_or(0) // 返回最终结果
}

#[test]
fn test_solve_express() {
    assert_eq!(solve_express("3*(3+1)".to_string()), 12); // 3 * 4 = 12
}

// input "1#1#+#3#+#4*"
// ouput  20
fn legal_exp(expr: String) -> i32 {
    let tokens: Vec<&str> = expr.split('#').collect();
    let mut stack: Vec<i32> = Vec::new();

    for token in tokens {
        match token {
            "+" | "-" | "*" => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                let res = match token {
                    "+" => b + a,
                    "-" => b - a,
                    "*" => b * a,
                    _ => unreachable!(),
                };
                stack.push(res);
            }
            _ => {
                let num = token.parse().unwrap();
                stack.push(num);
            }
        }
    }

    stack.pop().unwrap()
}

#[test]
fn test_legal_exp() {
    let str = "1#1#+#3#+#4#*".to_string();
    assert_eq!(legal_exp(str), 20);
}
// 入栈顺序 1 2 3 4 5  判断是否可以为 2 5 4 3 1 的顺序出栈
//  比如 入栈 1 2 出栈 2 在入栈 3 4 5 出栈 5 4 3 1
//  1 2 3 4 5  | 2 5 4 3 1
//  1 2 3 4 5  | 3 4 5 2 1
// 编写一个函数记录2个stack  判断后一个stack 是否当成第一个数组的出栈顺序
// fn is_legal_stack(input_stack: Vec<i32>, mut output_stack: Vec<i32>) -> bool {
//     // 如果输入输出长度不匹配，直接返回false
//     if input_stack.len() != output_stack.len() {
//         return false;
//     }

//     let mut tmp_stack = Vec::new(); // 临时栈

//     for input_value in input_stack {
//         // 检查输入值是否匹配当前输出序列顶部
//         if Some(&input_value) == output_stack.last() {
//             output_stack.pop();

//             // 检查临时栈与输出序列是否能继续匹配
//             while !tmp_stack.is_empty() && !output_stack.is_empty() {
//                 if let (Some(tmp_top), Some(output_top)) = (tmp_stack.last(), output_stack.last()) {
//                     if tmp_top == output_top {
//                         tmp_stack.pop();
//                         output_stack.pop();
//                     } else {
//                         break;
//                     }
//                 } else {
//                     break;
//                 }
//             }
//         } else {
//             tmp_stack.push(input_value);
//         }
//     }

//     // 处理临时栈中剩余元素
//     while !tmp_stack.is_empty() && !output_stack.is_empty() {
//         if tmp_stack.last() == output_stack.last() {
//             tmp_stack.pop();
//             output_stack.pop();
//         } else {
//             return false;
//         }
//     }

//     // 两个栈必须都为空
//     tmp_stack.is_empty() && output_stack.is_empty()
// }

fn is_legal_stack(input_stack: Vec<i32>, output_stack: Vec<i32>) -> bool {
    // 如果长度不相等，直接返回false
    if input_stack.len() != output_stack.len() {
        return false;
    }

    let mut stack = Vec::new(); // 辅助栈
    let mut j = 0; // 输出序列的索引

    for &num in input_stack.iter() {
        // 当前元素入栈
        stack.push(num);

        // 对 入栈的元素和 输出栈的原因比对，一样就POP
        while !stack.is_empty() && stack.last() == Some(&output_stack[j]) && j < output_stack.len()
        {
            stack.pop();
            j += 1;
        }
    }

    // 最终栈必须为空，且输出序列已全部匹配
    stack.is_empty() && j == output_stack.len()
}
#[test]
fn test_is_legal_stack() {
    let input_stack = vec![1, 2, 3, 4, 5];
    let output_stack = vec![3, 4, 5, 2, 1];

    assert!(is_legal_stack(input_stack, output_stack))
}

//  [2, 1, 5, 3, 4]     [5, 4, 3, 1, 2]
fn largest_permutation(stack_ops: Vec<i32>) -> Vec<i32> {
    let n = stack_ops.len();
    if n == 0 {
        return Vec::new();
    }

    // 构建后缀最大值数组
    let mut suffix_max = vec![i32::MIN; n + 1];
    for i in (0..n).rev() {
        suffix_max[i] = suffix_max[i + 1].max(stack_ops[i]);
    }

    let (mut stack, mut result) = (Vec::new(), Vec::new());
    for (i, &num) in stack_ops.iter().enumerate() {
        stack.push(num);
        // 当栈顶元素≥后序最大值时持续出栈
        while let Some(&top) = stack.last() {
            if top >= suffix_max[i + 1] {
                result.push(stack.pop().unwrap());
            } else {
                break;
            }
        }
    }
    // 剩余元素直接出栈
    result.extend(stack.into_iter().rev());
    result
}

#[test]
fn test_max_stack_output() {
    let input_stack = vec![2, 1, 5, 3, 4];
    println!("{:?}", largest_permutation(input_stack));
}
