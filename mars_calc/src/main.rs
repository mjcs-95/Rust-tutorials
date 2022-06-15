use std::io;

fn main() {
    println!("Enter your weight (in Kg): ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut weight: f32 = input.trim().parse().unwrap();
    weight = calculate_weight_on_mars(weight);
    println!("My weight on mars would be {} kg", weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    return (weight / 9.81) * 3.711;
}