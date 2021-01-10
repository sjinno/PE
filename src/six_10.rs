use std::cmp::Ordering::*;

// Ah... Probably I should use BigUint...
pub fn largest_product_in_a_series(n: u8) -> u64 {
    let filename = "inputs/input8.txt";
    let lines = readfile!(filename);
    // println!("{:?}", lines);
    // for line in lines.iter() {
    //     println!("{}", line);
    // }
    let length = lines[0].len();
    // println!("{}", length);
    match length.cmp(&(n as usize)) {
        Less => println!("The input size should not be smaller than n."),
        Equal => {
            let mut v = Vec::new();
            lines.iter().for_each(|line| {
                let product = line
                    .chars()
                    .into_iter()
                    .fold(1, |acc, x| acc * x.to_digit(10).unwrap());
                v.push(product);
            });
            return (*v.iter().max().unwrap()).into();
        }
        _ => {}
    }
    0
}
