//Convert temperatures between Fahrenheit and Celsius.

use std::io;

fn main() {
    println!("Input temperature in Celsius to convert to Fahrenheit");

    let mut c_temp = String::new();
    
    io::stdin()
        .read_line(&mut c_temp)
        .unwrap();

    let c_temp: f32 = c_temp.trim().parse().unwrap();

    let c_to_fahr = c_temp * 9.0 / 5.0 + 32.0;
    println!("{} Celsius is {} Fahrenheit",c_temp, c_to_fahr);

    //Convert temp for Fahr to Cel
    println!("Input temperature in Fahrenheit to convert to Celsius");

    let mut f_temp = String::new();
    io::stdin()
        .read_line(&mut f_temp)
        .unwrap();

    let f_temp: f32 = f_temp.trim().parse().unwrap();

    let fahr_to_c = (f_temp - 32.0) * 5.0 / 9.0;
    println!("{} Fahrenheit is {} Celsius", f_temp, fahr_to_c);
}
