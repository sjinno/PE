pub fn multiples_of_3_and_5(bound: u32) -> u32 {
    // let v: Vec<u32> = (3..bound)
    //     .into_iter()
    //     .filter(|n| n % 3 == 0 || n % 5 == 0)
    //     .collect();
    // println!("{:?}", v);
    // v.iter().sum::<u32>()
    (3..bound)
        .into_iter()
        .filter(|n| n.rem_euclid(3) == 0 || n.rem_euclid(5) == 0)
        .sum()
}
