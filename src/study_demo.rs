use std::io::{self, BufRead};
/// 学习读取一段连续的输入放入到一个vec，然后按照自定义输出
/// 学习读取一行然后根据自定义输出
/// 学习循环每读取一行然后根据自定义输出
///
/// 自定义：对整数类型加1 对浮点类型保留2位 其他按字符串输出
pub fn study_demo_1() {
    let stdin = io::stdin();
    let mut lines = Vec::new();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            break;
        }
        lines.push(line);
    }
    for line in lines {
        if let Ok(num) = line.trim().parse::<i64>() {
            println!("{}", num + 1);
        } else if let Ok(num) = line.trim().parse::<f64>() {
            println!("{:.2}", num);
        } else {
            println!("{}", line.trim());
        }
    }
}

pub fn study_demo_2() {
    let stdio = io::stdin();
    let mut buf = String::from("");

    loop {
        buf.clear();
        let _ = stdio.lock().read_line(&mut buf);
        if buf.trim().eq_ignore_ascii_case("exit") {
            break;
        }
        if let Ok(num) = buf.trim().parse::<i64>() {
            println!("{}", num + 1);
        } else if let Ok(num) = buf.trim().parse::<f64>() {
            println!("{:.2}", num);
        } else {
            println!("{}", buf);
        }
    }
}

pub fn study_demo_3() {
    let stdio = io::stdin();

    for line in stdio.lock().lines() {
        let line = line.unwrap();
        if line.trim().eq_ignore_ascii_case("exit") {
            break;
        }
        if let Ok(num) = line.trim().parse::<i64>() {
            println!("{}", num + 1);
        } else if let Ok(num) = line.trim().parse::<f64>() {
            println!("{:.2}", num);
        } else {
            println!("{}", line.trim())
        }
    }
}
/*
input: 1 2
output 3
*/
pub fn study_demo_4() {
    let stdin = io::stdin();
    let mut input = String::from("");
    let _ = stdin.lock().read_line(&mut input);
    let mut result = 0;
    for num in input.split(" ").into_iter() {
        if let Ok(num) = num.trim().parse::<i32>() {
            result += num
        }
    }
    println!("{}", result);
}
/*
input 10433 280
output (280/10433)*100%  =  2.684%
*/
pub fn study_demo_5() {
    let stdin = io::stdin();
    let mut input = String::from("");
    let _ = stdin.lock().read_line(&mut input);

    // let nums: Vec<f64> = input
    //     .split_whitespace()
    //     .filter_map(|s| s.trim().parse::<f64>().ok())
    //     .collect();
    // println!("{:.2}%", nums[1] / nums[0] * 100.0);

    let mut num = input.split_whitespace().into_iter();
    let num1 = num.next();
    let num2 = num.next();
    if let (Some(a), Some(b)) = (num1, num2) {
        if let (Ok(a), Ok(b)) = (a.parse::<f64>(), b.parse::<f64>()) {
            if a != 0.0 {
                println!("{:.2}%", b / a * 100.0);
            } else {
                println!("被除数为 0");
            }
        }
    }
}
/*
input : 15 2
output: 7 1   15/2=7  15%2=1
7 1
*/
pub fn study_demo_6() {
    let stdin = io::stdin();
    let mut input = String::from("");

    let _ = stdin.lock().read_line(&mut input);
    let nums: Vec<f64> = input
        .split_whitespace()
        .filter_map(|x| x.parse::<f64>().ok())
        .collect();
    println!("{} {}", (nums[0] / nums[1]) as i64, nums[0] % nums[1]);
}
/*
整数的各位 2个 %
input 114  output 4  114 % 100 =14  14%10 4
*/
pub fn study_demo_7() {
    let stdin = io::stdin();
    let mut input = String::from("");

    let _ = stdin.lock().read_line(&mut input);

    if let Ok(input) = input.trim().parse::<i32>() {
        let result = ((input % 100) % 10).abs();
        println!("{}", result);
    }
}
/*
整数的十位
input 114  output 4  114 /10 =14   14%10 4
*/
pub fn study_demo_8() {
    let stdin = io::stdin();
    let mut input = String::from("");
    let _ = stdin.read_line(&mut input);

    if let Ok(num) = input.trim().parse::<i32>() {
        let result = ((num / 10) % 10).abs();
        println!("{}", result);
    }
}
/*
平方根 向下取整
*/
pub fn study_demo_9() {
    let stdin = io::stdin();
    let mut input = String::from("");
    let _ = stdin.read_line(&mut input);

    if let Ok(num) = input.trim().parse::<f64>() {
        let result = num.sqrt().floor() as i32;
        println!("{}", result);
    }
}

/*
反向输出一个 4位数
input 1000  output 0001
 */
pub fn study_demo_10() {
    let stdin = io::stdin();
    let mut input = String::from("");

    let _ = stdin.read_line(&mut input);
    let mut result = "".to_string();
    for ch in input.trim().chars().rev() {
        result.push(ch);
    }
    println!("{}", result);
}
/*
温标转换 C = K -273.15  F = C * 1.8 +32
输入 K  要求返回 F
*/
pub fn study_demo_11() {
    let stdin = io::stdin();
    let mut input = String::from("");
    let _ = stdin.read_line(&mut input);

    let input_k = input.trim().parse::<f64>();
    if let Ok(k) = input_k {
        let f = (k - 273.15) * 1.8 + 32.0;
        println!("{:.9}", f);
    }
}
/*
dM -> qrt(pow(x1-x2,2)+pow(y1-y2,2));
dE -> fabs(x1-x2)+fabs(y1-y2);
fabs(dM - dE);
*/
pub fn study_demo_12() {
    let stdin = io::stdin();
    let mut vec = vec![];

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            break;
        }
        vec.push(line);
    }
    let col1: Vec<f64> = vec[0]
        .split_whitespace()
        .filter_map(|s| s.trim().parse::<f64>().ok())
        .collect();
    let col2: Vec<f64> = vec[1]
        .split_whitespace()
        .filter_map(|s| s.trim().parse::<f64>().ok())
        .collect();

    let (x1, y1) = (col1[0], col1[1]);
    let (x2, y2) = (col2[0], col2[1]);

    let d_m = ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt();
    let d_e = (x1 - x2).abs() + (y1 - y2).abs();
    let diff = (d_m - d_e).abs();

    println!("{:.6}", diff);
}
/*
输入 1234  输出 10
*/
pub fn study_demo_13() {
    let stdin = io::stdin();
    let mut input = String::from("");

    let _ = stdin.read_line(&mut input);
    let mut result = 0;

    for ch in input.trim().chars() {
        result += ch.to_digit(10).expect("Invalid digit") as i32;
    }
    println!("{}", result)
}
