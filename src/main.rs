#[macro_use]
mod macros;

// mod first_5;
mod six_10;

fn main() {
    // 1 - 5
    // println!("{}", first_5::multiples_of_3_and_5(1000));
    // println!("{}", first_5::even_fib_numbers(4_000_000));
    // println!("{}", first_5::largest_prime_factor(600_851_475_143));

    // 6 - 10
    println!("{}", six_10::largest_product_in_a_series(13));
}
