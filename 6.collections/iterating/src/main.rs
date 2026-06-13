use std::collections::HashMap;
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let sum = sum_of_elements(v);
    println!("The sum of the elements is: {}", sum);

    fn sum_of_elements(v: Vec<i32>) -> i32 {
        v.iter().sum()
    }

    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let even_numbers: Vec<_> = get_even_numbers(vec);

    println!("The even numbers are: {:?}", even_numbers);

    fn get_even_numbers(vec: Vec<i32>) -> Vec<i32> {
        let filter: Vec<_> = vec.into_iter()
                                .filter(|n| *n % 2 == 0)
                                .collect();
        return filter;
    }

    let grades = HashMap::from([
        ("Alice", 75),
        ("Bob", 55),
        ("John", 60),
        ("Mary", 90),
        ("Rob", 80)
    ]);

    calculate_average(grades);

    fn calculate_average(grades: HashMap<&str, i32>) {
        let mut value_sum = 0;
        let mut value_count = 0;
        for (_key, value) in &grades {
            value_sum += value;
            value_count += 1;
        }
        let average: f64 = value_sum as f64 / value_count as f64;
        println!("Average grade is: {:.2}", average);
    }

}