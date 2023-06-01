fn main() {
    println!("Hello, world!");

    let x = 1;
    let y = 2;
    let sum = sum::sum(x, y);
    println!("{} + {} = {}", x, y, sum);

    let subtract = subtract::subtract(x, y);
    println!("{} - {} = {}", x, y, subtract);
}
