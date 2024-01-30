fn process_numbers(numbers: &Vec<i32>) -> i32 {
    numbers.iter().sum()
}

pub fn main_very_hard() {
    let my_numbers = vec![1, 2, 3, 4, 5];

    let sum = process_numbers(&my_numbers);

    println!("Sum of numbers: {}", sum);
    println!("Original numbers: {:?}", my_numbers);
}
