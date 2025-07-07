use std::io::BufRead;

pub fn study_demo_20() {
    let stdin = std::io::stdin();
    let mut input = String::new();
    stdin.lock().read_line(&mut input).expect("except a number");
    if let Ok(num) = input.trim().parse::<i32>() {
        let result = num % 7 + 1;
        println!("{}", result);
    }
}

// 判断闰年
// n 能被 4整除 不能被 100 整除 (a%4==0&&a%100!=0)||a%400==0
pub fn study_demo_21() {
    let stdin = std::io::stdin();
    let mut input = String::new();
    stdin.lock().read_line(&mut input).expect("need a number");
    if let Ok(num) = input.trim().parse::<i32>() {
        if (num % 4 == 0 && num % 100 != 0) || num % 400 == 0 {
            println!("{}", true);
        } else {
            println!("{}", false);
        }
    };
}
// input 1 2   2  1
// output <     >

pub fn study_demo_22() {
    let mut input = String::new();
    std::io::stdin()
        .lock()
        .read_line(&mut input)
        .expect("expect a string a space b");
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();
    if nums[0] > nums[1] {
        println!(">");
    } else {
        println!("<");
    }
}
// input n      n 是奇数    n是偶数
// output f(n)  3n + 1      n / 2
pub fn study_demo_23() {
    let mut input = String::new();
    std::io::stdin()
        .lock()
        .read_line(&mut input)
        .expect("expect a number");
    if let Ok(num) = input.trim().parse::<i32>() {
        if num % 2 == 0 {
            println!("{}", num / 2);
        } else {
            println!("{}", num * 3 + 1);
        }
    }
}
// input  n
// output yes or no
// explain  n > 50 && n % 2 == 0
pub fn study_demo_24() {
    let mut input = String::new();
    std::io::stdin()
        .lock()
        .read_line(&mut input)
        .expect("expect a number");

    if let Ok(num) = input.trim().parse::<i32>() {
        if num > 50 && num % 2 == 0 {
            println!("{}", "yes");
        } else {
            println!("{}", "no");
        }
    }
}
// input a b c
// output YES NO
// explain if (a + b +c ) /3 > 60  ? NO : YES
pub fn study_demo_25() {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("expect 3 nums");

    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();
    let sum_number: i32 = nums.iter().sum();
    if sum_number / 3 > 60 {
        println!("NO");
    } else {
        println!("YES");
    }
}
// input a b c
// output max_val min_val
pub fn study_demo_26() {
    let mut input = String::new();
    std::io::stdin()
        .lock()
        .read_line(&mut input)
        .expect("expect 3 nums");
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();
    let max_val = nums.iter().max();
    let min_val = nums.iter().min();
    // match (max_val, min_val) {
    //     (Some(a), Some(b)) => {
    //         println!("max val is: {}", a);
    //         println!("min val is: {}", b);
    //     }
    //     _ => (),
    // }
    if let (Some(&a), Some(&b)) = (max_val, min_val) {
        println!("max val is: {}", a);
        println!("min val is: {}", b);
    }
}

// 3-5 spring  6-8 summer 9-11 actumn 12-2 womter
// input YYYYMM
pub fn study_demo_27() {
    let mut input = String::new();
    std::io::stdin()
        .lock()
        .read_line(&mut input)
        .expect("not a yyyymm");

    let mm = &input[4..].trim().parse::<i32>().unwrap();
    match mm {
        3 | 4 | 5 => println!("spring"),
        6 | 7 | 8 => println!("summer"),
        9 | 10 | 11 => println!("actumn"),
        12 | 1 | 2 => println!("womter"),
        _ => (),
    }
}
// input  2  1 1  2 2
// output     2    4
pub fn study_demo_28() {
    let mut vec = Vec::new();

    for line in std::io::stdin().lines() {
        if let Ok(str) = &line {
            if str.trim().is_empty() {
                break;
            }
        }
        vec.push(line);
    }

    for num in vec.iter().skip(1) {
        if let Ok(rs) = num {
            let result: i32 = rs
                .trim()
                .split_whitespace()
                .filter_map(|x| x.parse::<i32>().ok())
                .sum();
            println!("{}", result);
        }
    }
}
// 0 0 不计算

pub fn study_demo_29() {
    // 存取输入
    let mut lines = Vec::new();
    for line in std::io::stdin().lines() {
        match line {
            Ok(s) if s.trim().is_empty() => break,
            Ok(s) => lines.push(s),
            Err(e) => {
                eprint!("读取错误{}", e);
                break;
            }
        }
    }

    for s in lines {
        let result: i32 = s
            .trim()
            .split_whitespace()
            .filter_map(|x| x.parse::<i32>().ok())
            .sum();
        // if result == 0 {
        //     continue;
        // }
        println!("{}", result);
    }
}

// 素数
pub fn study_demo_30() {
    let mut lines = Vec::new();
    for line in std::io::stdin().lines() {
        match line {
            Ok(s) if s.trim().is_empty() => break,
            Ok(s) => lines.push(s),
            Err(e) => {
                eprint!("error {}", e);
                break;
            }
        }
    }
    for num in lines.iter().skip(1) {
        let r_num = num.trim().parse::<i32>().ok();
        if let Some(ok_num) = r_num {
            let output = is_prime(ok_num);
            println!("{}", output);
        };
    }
}

fn is_prime(n: i32) -> bool {
    // 处理小于 2 和 2 的情况
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    // 排除偶数（除 2 外）
    if n % 2 == 0 {
        return false;
    }

    // 检查奇数因子（从 3 到平方根）
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        // 每次加 2，跳过偶数
        i += 2;
    }

    true
}
// 数列 如果 n 为奇数 (n +1) / 2 偶数： -n/2
pub fn study_demo_31() {
    let mut input = String::new();
    std::io::stdin()
        .lock()
        .read_line(&mut input)
        .expect("not a vaild number");

    let result_parse = input.trim().parse::<i32>();
    if let Ok(result) = result_parse {
        if result % 2 == 0 {
            println!("{}", -result / 2);
        } else {
            println!("{}", (result + 1) / 2)
        }
    }
}

// input n
// output 1/1  + 1/2 + 1/3 + 1/4 + ... 1/n

fn add_n(n: i32) -> f64 {
    // if n == 1 {
    //     return 1.0;
    // }
    // let prev = add_n(n - 1);
    // prev + (1.0 / n as f64)
    (1..n).map(|i| 1.0 / i as f64).sum()
}
pub fn study_demo_32() {
    let mut buf = String::new();
    std::io::stdin()
        .lock()
        .read_line(&mut buf)
        .expect("not a valid number");

    if let Ok(num) = buf.trim().parse::<i32>() {
        let result = add_n(num);
        println!("{:.6}", result);
    }
}
// biggest difference
// in put 10 2 3 5     max_val - min_val  = 10 -2 output 8
pub fn study_demo_33() {
    let mut lines = Vec::new();

    for line in std::io::stdin().lock().lines() {
        match line {
            Ok(s) if s.trim().is_empty() => break,
            Ok(s) => lines.push(s),
            Err(e) => {
                eprintln!("error input {}", e);
                break;
            }
        }
    }
    let nums: Vec<i32> = lines[0]
        .split_whitespace()
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();
    let max_val = nums.iter().max();
    let min_val = nums.iter().min();
    match (max_val, min_val) {
        (Some(a), Some(b)) => println!("{}", a - b),
        _ => (),
    }
}

// input  n
// output 1 + 1 +2 + 1+2+3 + 1+2+3+...n
fn sum_series(n: i32) -> i32 {
    // 内部递归函数计算累加和：1+2+...+k
    fn sum_up_to(k: i32) -> i32 {
        if k <= 0 { 0 } else { sum_up_to(k - 1) + k }
    }

    if n <= 0 {
        0
    } else {
        // 递归计算当前项（1+2+..+n）并加上前n-1项的和
        sum_series(n - 1) + sum_up_to(n)
    }
}
#[allow(dead_code)]
fn fibonacci_iterative(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    let (mut a, mut b) = (0, 1);
    for _ in 1..n {
        let next = a + b;
        a = b;
        b = next;
    }
    b
}
pub fn study_demo_34() {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("not valid num");
    let num_n = input.trim().parse::<i32>();
    if let Ok(n) = num_n {
        println!("{}", sum_series(n));
    }
}
// input    -100     222
// output    1+0+0   2+2+2
pub fn study_demo_35() {
    let mut input = String::new();
    std::io::stdin()
        .lock()
        .read_line(&mut input)
        .expect("not vaild number");

    let result: u32 = input.trim().chars().filter_map(|x| x.to_digit(10)).sum();
    println!("{}", result);
}
// input 9
// output 123 567 9  exclude 4*n
pub fn study_demo_36() {
    let mut inuput = String::new();
    std::io::stdin()
        .read_line(&mut inuput)
        .expect("not vaild number");
    let num = inuput.trim().parse::<i32>();

    if let Ok(n) = num {
        (0..n)
            .into_iter()
            .filter(|&x| x % 4 != 0)
            .for_each(|x| println!("{}", x));
    }
}
// input n
// output n=1 = 0  n = 2|3 =1 n>4 0 + 2*1 + 1 =3
pub fn study_demo_37() {
    let mut input = String::new();
    std::io::stdin()
        .lock()
        .read_line(&mut input)
        .expect("not vaild number");
    let result_parse_num = input.trim().parse::<i32>();
    if let Ok(num) = result_parse_num {
        match num {
            0 => println!("0"),
            2 | 3 => println!("1"),
            _ if num >= 4 => {
                let (mut a, mut b, mut c) = (0, 1, 1);
                let mut current;
                for _ in 4..=num {
                    current = a + 2 * b + c;
                    a = b;
                    b = c;
                    c = current;
                }
                println!("{}", c);
            }
            _ => (),
        }
    }
}
