use std::io;

fn main() {
    // Read a line from standard input
    println!("Enter a number: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse the input into the desired type
    let number: i32 = input.trim().parse().expect("Invalid input");

    // Use the input
    println!("You entered: {}", number);

    // Doing Some Arithmetic Operations

    let num1 = 20;
    let num2 = 30;
    let num3 = 40;
    let num4 = 50;

    let sum = num1 + num2 + num3 + num4;
    println!("Sum: {}", sum);

    let avg = sum / 4;
    println!("Average: {}", avg);

    let subtract = num4 - num3;
    println!("Subtract: {}", subtract);

    let multiply = num1 * num2 * num3 * num4;
    println!("Multiply: {}", multiply);

    let divide = num4 / num1;
    println!("Divide: {}", divide);

    let remainder = num4 % num1;
    println!("Remainder: {}", remainder);

    let x = 50u8;
    let y = 30u8;
    let z = x + y;
    println!("z: {}", z); // the value of z will be in range of 0 to 255 because the type of z is u8

    // let a = 50u8;
    // let b = 30u8;
    // let c = b-a;
    // println!("C: {}", c); // the will give an error because the type of c is u8 and the value which returns is -20 which is not in range of u8

    // let p = 50i32;
    // let q = 30u8;
    // let r = p - q;
    // println!("R: {}", r); // This will give an error in output because the type of p is i32 and the type of q is u8 and the type of r is i32 and the value of r is -20 which is not in range of u8

    // we can not do arethmetic oprations on explicitly different types of variables    

}










 