fn main() {
    println!("Hello, world!");
    println!("{}", fibonacci(100, 0, 1, 1));
}

fn fibonacci(index: usize, actual_index: usize, old_val: u128, new_val: u128) -> u128{
    if index == actual_index{
        return new_val;
    }
    fibonacci(index, actual_index + 1, new_val, old_val + new_val)
}