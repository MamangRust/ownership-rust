fn process_vector(v: Vec<i32>) -> Vec<i32> {
    v.into_iter().map(|x| x * 2).collect()
}

pub fn main_medium() {
    let numbers = vec![1, 2, 3, 4, 5];

    let doubled_numbers = process_vector(numbers);

    println!("Doubled numbers: {:?}", doubled_numbers);
}
