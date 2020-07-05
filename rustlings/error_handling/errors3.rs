use std::num::ParseIntError;
use std::process::exit;
fn main() {
    let mut tokens = 100;
    let pretend_user_input = "8";

    match(total_cost(pretend_user_input)){
        Ok(n)=>{
            if n > tokens {
                println!("You can't afford that many!");
            } else {
                tokens -= n;
                println!("You now have {} tokens.", tokens);
            }
        },
        Err(s)=>{
            println!("{} error",s);
            exit(1);
        }
    }
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    match item_quantity.parse::<i32>(){
        Ok(qty)=>Ok(qty * cost_per_item + processing_fee),
        Err(ParseIntError)=>Err(ParseIntError)
    }
}
