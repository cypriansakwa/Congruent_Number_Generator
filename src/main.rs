fn main() {
    let count = 25; 
// Specify the +ve and negative numbers to be generated

    let positive_numbers = generate_positive_congruent_numbers(3, 4, count);
    let negative_numbers = generate_negative_congruent_numbers(3, 4, count);

    println!("Positive numbers congruent to 123457638964 mod 713: {:?}", positive_numbers);
    println!("Negative numbers congruent to 123457638964 mod 713: {:?}", negative_numbers);
}

fn generate_positive_congruent_numbers(congruent: i64, modulus: i64, count: usize) -> Vec<i64> {
    (0..count).map(|k| congruent + k as i64 * modulus).collect()
}

fn generate_negative_congruent_numbers(congruent: i64, modulus: i64, count: usize) -> Vec<i64> {
    (1..=count).map(|k| congruent - k as i64 * modulus).collect()
}
