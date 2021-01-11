#[macro_use]
mod macros;

mod first_5;
mod six_10;
mod twentyone_25;

fn main() {
    // 1 - 5
    // println!("{}", first_5::multiples_of_3_and_5(1000));
    // println!("{}", first_5::even_fib_numbers(4_000_000));
    // println!("{}", first_5::largest_prime_factor(600_851_475_143));
    // println!("{}", first_5::largest_palindrome_product(3, 4));

    // // 6 - 10
    // println!("{}", six_10::largest_product_in_a_series(13));
    // println!("{}", six_10::special_pythagorean_triplet(1000));

    // 21 - 25
    println!(
        "{}",
        twentyone_25::lexicographic_permutations(10, 1_000_000)
    );
    // for i in 1..7 {
    //     println!(
    //         "Count {}: {}.",
    //         i,
    //         twentyone_25::lexicographic_permutations(3, i)
    //     );
    // }
}
