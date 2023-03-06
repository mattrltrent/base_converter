// Converts from base 2 - 36

// todo: flag for explaining to base 10 and to other bases
// todo: protect against variable overflow
// todo: add help menu
// todo: add conversion to ensure all uppercase

pub mod errors;
pub mod conversions;
use errors::ErrVariants;

use crate::conversions::ValueConverter;

fn main() {
    let from_base = 22;
    let to_base = 16;
    let explain: bool = true;
    match calc(from_base, to_base, explain) {
        Ok(answer) => {
            if !explain {
                println!("{answer}")
            }
        },
        Err(err) => println!("Error: {}\nRun 'convert help' for help!", err.message())
    }
}

fn calc(from_base: u32, to_base: u32, explain: bool) -> Result<String, ErrVariants>{

    let converter = ValueConverter::new();
    
    let input = String::from("1B6");

    if input.is_empty() {return Err(ErrVariants::EmptyInput("No input provided".to_string()))}



    let mut nums: Vec<u32> = vec![];

    for c in input.chars() {
        match converter.from_char(&c) {
            Ok(num) => if num < from_base {nums.push(num)} else {return Err(ErrVariants::CharacterNotAbidingByBase(format!("Character '{}' invalid given your original base '{}'", c, from_base)))}
            Err(err) => return Err(err),
        }
    };

    // todo: command to show conversion table
    if explain {
        println!("CONVERTING {} (BASE {}) TO BASE {}\n", {&input}, {&from_base}, {to_base});
        println!("First, map all characters in your base {} string {} to their corresponding numeric value. Run 'convert table' to view the table.\n", {from_base}, {&input});
        for idx in 0..nums.iter().len() {
            println!("{} -> {}", input.chars().nth(idx).unwrap(), nums[idx]);
        }
        println!("")
    }

    let mut remainders: Vec<u32> = vec![];
    let mut work: Vec<String> = vec![];

    if explain {println!("Then, take the summation of (<each_mapped_result> * <base_you_are_converting_from>)^(<the_digit_position>) like so:\n")}

    let decimal = base_n_nums_to_decimal(from_base, &nums, explain);

    if explain {println!("Next, we take this base 10 number ({}) and do the Euclidean Algorithm on it. This means we take our number, divide it by the base we're converting it to, and repeatedly find the result when we do the equation again with the quotient of our last computed result. We then repeat this equation until our quotient is 0. The whole time we're taking note of the remainder at each step:\n", {&decimal});}

    let new_base_nums = decimal_to_base_n(to_base, decimal, &mut remainders, &mut work, explain);

    if explain {println!("\nNext, we take each of the remainders from the equation above: {:?}\n",{&new_base_nums})}

    let mut new_base_str: Vec<char> = vec![];

    for idx in 0..new_base_nums.len() {
        match converter.from_num(&new_base_nums[new_base_nums.len() - 1 - idx]) {
            Ok(s) => new_base_str.push(s),
            Err(err) => return Err(err),
        }
    }

    

    if explain {
        println!("Finally, We then map each of these values using our conversion table to their corresponding characters. Run 'convert table' to view the table.\n");
        for idx in 0..new_base_nums.iter().len() {
            println!("{} -> {}", new_base_nums[idx], new_base_str[idx]);
        }
        println!("\nThis is our final result => {}", {new_base_str.iter().collect::<String>()});
        println!("\nWe converted {} (base {}) => {} (base {})", {input}, {from_base}, {new_base_str.iter().collect::<String>()}, {to_base});
    }

    Ok(new_base_str.iter().collect::<String>())
}

// todo: fix possible panic on usize -> u32 conversion
fn base_n_nums_to_decimal(base: u32, nums: &Vec<u32>, explain: bool) -> u32 {

    let mut work = String::from("");

    let mut total = 0;
    for idx in 0..nums.len() {
        total += base.pow(idx as u32) * nums[nums.len() - 1 - idx];
        work = format!("{}{} {}", format!("({}x{})^{}", nums[nums.len() - 1 - idx], base, idx), if idx != 0 {" +"} else {""}, work);
    }
    work += &format!("= {} (base 10)\n", total);
    if explain {println!("{work}")}
    total
}

fn decimal_to_base_n<'a>(base: u32, num: u32, remainders: &'a mut Vec<u32>, work: &'a mut Vec<String>, explain: bool) -> &'a mut Vec<u32> {
    let (quotient, remainder) = divide(num, base);
    remainders.push(remainder);
    work.push(format!("{} / {} = {} + {}", {num}, {base}, {quotient}, {remainder}));
    if quotient == 0 {
        if explain {
            for equation in work {
                println!("{equation}");
            }
        }
        return remainders
    } else {
        return decimal_to_base_n(base, quotient, remainders, work, explain);
    }
}

fn divide(dividend: u32, divisor: u32) -> (u32, u32) {
    (dividend / divisor, dividend % divisor)
}




