use std::env;

const PROTEIN: f64 = 2.2; // 1.6 - 2.2

const CARBOHYDRATES: f64 = 1.7; // 1.5 - 2.0

const FAT: f64 = 0.65; // 0.4 - 0.8

fn main() {
    let weight: Vec<f64> = env::args()
        .skip(1)
        .map(|a| a.parse::<f64>().unwrap_or(0.0))
        .collect();

    let weight = weight[0];

    println!("Provided weight: {}kg", weight);
    println!("protein: {:.0}g", weight * PROTEIN);
    println!("carbohydrates: {:.0}g", weight * CARBOHYDRATES);
    println!("fat: {:.0}g", weight * FAT);
}
