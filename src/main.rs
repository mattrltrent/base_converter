// Converts from base 2 - 36

// todo: flag for explaining to base 10 and to other bases
// todo: protect against variable overflow
// todo: add help menu

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
    let to_base = 16;

    let mut nums: Vec<u32> = vec![];

    for c in input.chars() {
        match converter.from_char(&c) {
            Ok(num) => if num < from_base {nums.push(num)} else {return Err(ErrVariants::CharacterNotAbidingByBase(format!("Character '{}' invalid given your base '{}'", c, from_base)))}
            Err(err) => return Err(err),
        }
    };

    let mut remainders: Vec<u32> = vec![];
    let new_base_nums = decimal_to_base_n(to_base, base_n_nums_to_decimal(from_base, &nums), &mut remainders);

    let mut new_base_str: Vec<char> = vec![];
    let new_base_nums_len = new_base_nums.len();

    for idx in 0..new_base_nums_len {
        match converter.from_num(&new_base_nums[new_base_nums_len - 1 - idx]) {
            Ok(s) => new_base_str.push(s),
            Err(err) => return Err(err),
        }
    }

    Ok(new_base_str.iter().collect::<String>())
}

// todo: fix possible panic on usize -> u32 conversion
fn base_n_nums_to_decimal(base: u32, nums: &Vec<u32>) -> u32 {
    let nums_len = nums.len();
    let mut total = 0;
    for idx in 0..nums_len {
        total += base.pow(idx as u32) * nums[nums_len - 1 - idx];
    }
    total
}

fn decimal_to_base_n(base: u32, num: u32, remainders: &mut Vec<u32>) -> &mut Vec<u32> {
    let (quotient, remainder) = divide(num, base);
    remainders.push(remainder);
    if quotient == 0 {
        return remainders
    } else {
        return decimal_to_base_n(base, quotient, remainders);
    }
}

fn divide(dividend: u32, divisor: u32) -> (u32, u32) {
    (dividend / divisor, dividend % divisor)
}




