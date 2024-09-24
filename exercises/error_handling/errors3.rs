// errors3.rs
//
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
// 这是一个尝试使用前一个练习中完成的 `total_cost` 函数的程序。
// 但它不起作用！为什么？我们应该怎么做来修复它？
//
// Execute `rustlings hint errors3` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

use std::num::ParseIntError;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many!");
        Err("Not enough tokens".into())
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
        Ok(())
    }
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
