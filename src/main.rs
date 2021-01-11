#[macro_use]
mod macros;

mod pe_001_005;
mod pe_006_010;
mod pe_021_025;

fn main() {
    // 1 - 5
    // println!("{}", pe_001_005::multiples_of_3_and_5(1000));
    // println!("{}", pe_001_005::even_fib_numbers(4_000_000));
    // println!("{}", pe_001_005::largest_prime_factor(600_851_475_143));
    // println!("{}", pe_001_005::largest_palindrome_product(3, 4));

    // // 6 - 10
    // println!("{}", pe_006_010::largest_product_in_a_series(13));
    // println!("{}", pe_006_010::special_pythagorean_triplet(1000));

    // 21 - 25
    println!("{}", pe_021_025::lexicographic_permutations(10, 1_000_000));
    // for i in 1..7 {
    //     println!(
    //         "Count {}: {}.",
    //         i,
    //         pe_021_025::lexicographic_permutations(3, i)
    //     );
    // }
}
