use std::io;
use std::process; //for the exit function

fn main() {

    loop {
        println!("Please input a temperature to convert. If you would like to cancel, do so at any time by typing 'exit'");

        let mut temp_to_convert = String::new();
        io::stdin().read_line(&mut temp_to_convert)
            .expect("Failed to read line");
        
        let temp_to_convert = temp_to_convert.trim();

        if temp_to_convert == "exit" || temp_to_convert == "EXIT" || temp_to_convert == "Exit" {
            process::exit(0x0100);
        }

        let temp_to_convert: u32 = match temp_to_convert.parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("And what format is that temperature in? Celsius or Fahrenheit?");

        let mut temp_format = String::new();

        io::stdin().read_line(&mut temp_format)
            .expect("Failed to read line");

        let temp_format = temp_format.trim().to_lowercase();

        if temp_format == "fahrenheit" || temp_format == "f" {
            let converted_temp = f_to_c(temp_to_convert);
            println!("{} degrees Fahrenheit is {} degrees Celsius", temp_to_convert, converted_temp);
        } else if temp_format == "celsius" || temp_format == "c" {
            let converted_temp = c_to_f(temp_to_convert);
            println!("{} degrees Celsius is {} degrees Fahrenheit", temp_to_convert, converted_temp);
        } else if temp_format == "exit" {
            process::exit(0x0100);
        } else {
            println!("Your input was invalid. Try again dummy.")
        }
    }
}

// don't forget to set a -> return type
fn f_to_c(temp_to_convert: u32) -> u32 {
    // C = (F - 32) * 5/9
    return (temp_to_convert - 32) * 5/9;
}


fn c_to_f(temp_to_convert: u32) -> u32 {
    // F = (9/5)C + 32
    return (temp_to_convert * (9/5)) + 32;
}