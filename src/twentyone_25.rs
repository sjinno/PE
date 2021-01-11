use std::char;

#[allow(dead_code)]
pub fn lexicographic_permutations(num_of_digits: usize, nth: usize) -> String {
    let mut digits: Vec<Option<char>> = (0..num_of_digits)
        .map(|n| char::from_digit(n as u32, 10))
        .collect();

    let mut count = num_of_digits;
    let mut ans: Vec<char> = Vec::with_capacity(num_of_digits);

    let mut rem = nth;
    while count != 1 {
        let divisor = (1..count).fold(1, |acc, x| acc * x);
        if divisor * num_of_digits == nth {
            return digits
                .into_iter()
                .rev()
                .map(|c| c.unwrap())
                .collect::<String>();
        }
        match (rem, divisor) {
            (r, d) if r == d => {
                let quo = r / d;
                ans.push(digits.remove(quo - 1).unwrap());
                // println!("{:?}", ans);
                let rev: Vec<char> = digits.into_iter().rev().map(|i| i.unwrap()).collect();
                ans = vec![ans, rev].into_iter().flatten().collect();
                // println!("{:?}", ans);
                let answer = ans.iter().collect::<String>();
                // println!("{:?}", answer);
                return answer;
            }
            _ => {
                let quo = rem.div_euclid(divisor);
                rem = rem.rem_euclid(divisor);
                match rem {
                    0 => {
                        ans.push(digits.remove(quo - 1).unwrap());
                        rem = quo;
                    }
                    _ => ans.push(digits.remove(quo).unwrap()),
                }
            }
        }
        count -= 1;
    }
    ans.push(digits.pop().unwrap().unwrap());
    // println!("{:?}", ans);
    ans.iter().collect::<String>()
}
