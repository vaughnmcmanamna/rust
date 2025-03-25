use std::io;

fn main() {
    println!("Enter Temp ");

    let mut value = String::new();

    io::stdin()
        .read_line(&mut value)
        .expect("expect number");

    let value: f32 = value.trim().parse().expect("Expect Number");
    
    println!("Your Temp {}", value);

    println!("Press C to convert to Celsius");
    println!("Press F to convert to Fahrenheit"); 
    
    let mut format = String::new();

    io::stdin()
        .read_line(&mut format)
        .expect("Expect char");

    
    let format:char = format.trim().parse().expect("Expect Char");

    if format.to_ascii_uppercase() == 'C' {
        let x = convert_to_celsius(value);
        println!("{x} degrees Celsius");
    }

    if format.to_ascii_uppercase() == 'F' {
        let y = convert_to_fahrenheit(value);
        println!("{y} degrees Fahrenheit");
    }

}

fn convert_to_celsius(value: f32) -> f32 {
    (value - 32.0) * 5.0/9.0
} 

fn convert_to_fahrenheit(value: f32) -> f32 {
    (value * 9.0/5.0) + 32.0
}
