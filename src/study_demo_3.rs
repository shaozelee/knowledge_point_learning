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
        let line_for_parse:Vec<&str> =  score_line.split_whitespace().collect();
        output_str.entry(line_for_parse[0]).or_insert(Vec::new()).push(line_for_parse[1..].join(" "));
    }
    let mut max_total = i32::MIN;
    let mut max_name = String::new();
    let mut max_score_line = String::new();
    for (key_name,value_scores) in output_str {
        for score_line in &value_scores {
            let current_total:i32 = score_line
                .split_whitespace()
                .filter_map(|x|x.parse::<i32>().ok())
                .sum();

            if current_total > max_total {
                max_total = current_total;
                max_name = String::from(key_name);
                max_score_line = String::from(score_line);
            }
        }

    }
    println!("{} {}",max_name,max_score_line);
}

struct Point{
    x:i32,
    y:i32
}
impl Point {
    pub fn new(x:i32,y:i32) -> Point {
        Point{x:x,y:y}
    }
}
fn point_distance(a:&Point,b:&Point) -> i32 {
    // 计算 x 坐标差的绝对值
    let dx = (a.x - b.x).abs();
    // 计算 y 坐标差的绝对值
    let dy = (a.y - b.y).abs();
    // 返回曼哈顿距离
    dx + dy
}
pub fn study_demo_3() {
    let point1 = Point::new(0,0);
    let point2 = Point::new(2,3);
    let distance = point_distance(&point1,&point2);
    println!("{}",distance);
}
#[allow(dead_code)]
fn is_excellent(score_1:f64,score_2:f64) -> bool {
    let score  = score_1 * 0.7 + score_2 * 0.3;
    if score > 60.0 {
        true
    }else {
        false
    }
}
#[test]
fn test_is_excellent() {
    assert!(is_excellent(70.0, 70.0));
}
// 序列操作
pub fn study_demo_4(){
    let mut vec:Vec<i32> = Vec::new();
    vec.push(0);
    vec.push(1);
    vec.push(5);
    vec.push(8);
    vec.push(10);
    vec.push(11);
    vec.pop();
    vec.get(2).unwrap_or(&0);
    vec.sort_by(|a,b| a.cmp(b) );
    println!("{:?}",vec);
    vec.sort_by(|a,b| b.cmp(a) );
    println!("{:?}",vec);
    let len = vec.len();
    println!("{:?}",len);
}

fn count_peak_point(vec:&Vec<i32>) -> i32 {
    let n = vec.len();
    if n == 0 {return 0};
    if n == 1 {return 1};

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
    count += vec.windows(3)
        .filter(|w|w[1] > w[0] && w[1] > w[2])
        .count() as i32;

    // 末尾计算
    if vec[vec.len()-1] > vec[vec.len()-2]  {
        count += 1;
    }
    count
}

#[allow(dead_code)]
fn count_peak_point_2(vec: &Vec<i32>) -> i32 {
    (0..vec.len()).fold(0, |count, i| {
        count + match i {
            0 if vec[i] > vec[i + 1] => 1, // 首元素
            i if i == vec.len() - 1 && vec[i] > vec[i - 1] => 1, // 尾元素
            i => (vec[i] > vec[i - 1] && vec[i] > vec[i + 1]) as i32 // 中间元素
        }
    })
}
#[test]
fn test_count_peak_point() {
    let vec = vec![0,1,2,1,4,5,4,7,8,9];
    assert_eq!(count_peak_point(&vec),3);
}
// how long takes
// input [150,180,170],[35,32,33,34],2,4  [150,180,170],[35->150<-32->180<-33->170<-34] 634
fn how_long_seconds_need(spend: Vec<i32>, dwell: Vec<i32>, start: usize, end: usize) -> i32 {
        dwell.iter().take(end).skip(start-1).sum::<i32>() +
        spend.iter().take(end-1).skip(start-1).sum::<i32>()
}
#[test]
fn test_how_long_seconds_need() {
    let spend_seconds = vec![150,180,170];
    let dwell_seconds = vec![35,32,33,34];
    let start = 2;
    let end = 4;
    println!("{}",how_long_seconds_need(spend_seconds,dwell_seconds,start,end));
}

// input [1,2,3]
// ouput "1,2,3"
#[allow(dead_code)]
fn vec_to_string(vec:Vec<i32>) -> String {
    vec.iter().map(|x|format!("{}", x)).collect::<Vec<String>>().join(",")
}
#[test]
fn test_vec_to_string() {
    println!("{}",vec_to_string(vec![1,2,3,4,5]));
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
    let vec = vec![1,2,3,4,5];
    assert_eq!(the_end_numbers(vec.clone(), 2), 4);
    assert_eq!(the_end_numbers(vec.clone(), 3), 3);
}
#[allow(dead_code)]
fn vec_multiplication_vec(nums1:Vec<i32>,nums2:Vec<i32>) -> i32 {
    nums1.into_iter().zip(nums2.into_iter()).fold(0, |acc,(num1,num2)|acc + (num1 * num2))
}
#[test]
fn test_vec_multiplication_vec() {
    let vec1 = vec![1,2,3,4,5];
    let vec2 = vec![6,7,8,9];
   println!("{}",vec_multiplication_vec(vec1,vec2));
}
// stack
#[allow(dead_code)]
fn study_stack_vec(){
    let mut stack = vec![1,2,3,4,5];
    stack.push(6);
    stack.push(7);
    stack.push(8);
    stack.pop();
    if let Some(num) = stack.get(stack.len()-1){
        println!("{}",num);
    }
    println!("{}",stack.len());
    println!("{:?}",stack);
}
#[test]
fn test_study_stack_vec() {
    study_stack_vec();
}
