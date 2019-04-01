use std::io;

fn main() {
  println!("Temperature Converter: Converting Fahrenheit to Celcius since 1885");
  println!("Please enter the temperature in Fahrenheit");

  let mut input = String::new();
  io::stdin().read_line(&mut input)
    .expect("Unable to read input");

  let fahrenheit: f64 = input.trim().parse()
    .expect("Was unable to parse the number");

  let celcius = convert(&fahrenheit);

  println!("{} Fahrenheit is {} in Celcius", fahrenheit, celcius);
}

fn convert(fahrenheit: &f64) -> f64 {
  (fahrenheit - 32.0) / 1.8
}
