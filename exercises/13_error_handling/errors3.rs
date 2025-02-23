// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?

use std::num::ParseIntError;

// Don't change this function.
fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}

// TODO: Fix the compiler error by changing the signature and body of the
// `main` function.
fn main() ->Result<(), String> {
    let mut tokens = 100;
    let pretend_user_input: &str = "8";

    // Don't change this line.
    let cost = total_cost(pretend_user_input).map_err(|e| e.to_string())?;

    if cost > tokens {
        println!("You can't afford that many!");
        Err("You can't afford that many!".to_string())
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
        Ok(())
    }
}
