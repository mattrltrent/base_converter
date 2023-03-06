// Converts from base 2 - 36
pub mod errors;
pub mod conversions;
use errors::ErrVariants;

use crate::conversions::ValueConverter;

fn main() {
    match calc() {
        Ok(answer) => println!("Answer: {answer}"),
        Err(err) => println!("Error: {}", err.message())
    }
}

fn calc() -> Result<String, ErrVariants>{

    let converter = ValueConverter::new();
    
    let input = String::from("1BG");
    let from_base = 22;

    let mut nums: Vec<u32> = vec![];

    for c in input.chars() {
        match converter.from_char(&c) {
            Ok(num) => if num >= from_base {nums.push(num)} else {return Err(ErrVariants::CharacterNotAbidingByBase(format!("Character '{}' invalid given your base '{}'", c, from_base)))},
            Err(err) => return Err(err),
        }
    };

    println!("Vec: {:?}", nums);
    todo!()
}


