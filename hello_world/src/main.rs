fn main() {
    println!("Hello, VisionInDream Computer System Research");

    let sum = 45 + 78;

    let difference = 69 - 34;

    let multiplication = 67 * 76;

    let division = 78.0 / 13.7;

    let floor_division = 2 / 3;

    let remainder = 45 % 6;

    println!("sum: {}", sum);
    println!("difference: {}", difference);
    println!("multiplication: {}", multiplication);
    println!("division: {}", division);
    println!("floor division: {}", floor_division);
    println!("remainder: {}", remainder);

    let x = if 6 > 78 { 6 } else { 78 };
    println!("x is {x}");

    for num in [1, 2, 3, 4, 5] {
        println!("{num}");
    }

    let mut y = 0;

    while y < 10 {
        println!("I Love the number {y}");
        y += 1;
    }
}
