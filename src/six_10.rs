use std::cmp::Ordering::*;

#[allow(dead_code)]
pub fn largest_product_in_a_series(n: u8) -> u64 {
    let filename = "inputs/input8.txt";
    let lines = readfile!(filename);
    // println!("{:?}", lines);
    // for line in lines.iter() {
    //     println!("{}", line);
    // }
    let oneline = lines.join("");
    let length = oneline.len();
    println!("{}", length);
    let mut largest = 0;
    match length.cmp(&(n as usize)) {
        Less => println!("The input size should not be smaller than n."),
        _ => {
            let vec = series(&oneline, n as usize, length);
            for v in &vec {
                if !v.chars().into_iter().any(|c| c == '0') {
                    let tmp = v
                        .chars()
                        .into_iter()
                        .fold(1, |acc, x| acc * (x as u64 - 48));
                    if tmp > largest {
                        println!("{:?} ===> {}", v, tmp);
                        largest = tmp;
                    }
                }
            }
        }
    }
    largest
}

fn series(digits: &str, len: usize, length: usize) -> Vec<String> {
    (0..length + 1 - len)
        .scan("".to_string(), |_, x| {
            let num = digits.get(x..x + len).unwrap();
            Some(num.to_string())
        })
        .collect::<Vec<String>>()
}
