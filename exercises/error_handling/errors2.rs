// errors2.rs
//
// Say we're writing a game where you can buy items with tokens. All items cost
// 5 tokens, and whenever you purchase items there is a processing fee of 1
// token. A player of the game will type in how many items they want to buy, and
// the `total_cost` function will calculate the total cost of the tokens. Since
// the player typed in the quantity, though, we get it as a string-- and they
// might have typed anything, not just numbers!
//
// Right now, this function isn't handling the error case at all (and isn't
// handling the success case properly either). What we want to do is: if we call
// the `parse` function on a string that is not a number, that function will
// return a `ParseIntError`, and in that case, we want to immediately return
// that error from our function and not try to multiply and add.
//
// There are at least two ways to implement this that are both correct-- but one
// is a lot shorter!
// 假设我们正在编写一个游戏，你可以用代币购买物品。所有物品的价格都是5个代币，
// 每次购买物品时都有1个代币的处理费。游戏玩家将输入他们想要购买的物品数量，
// `total_cost` 函数将计算代币的总成本。然而，由于玩家输入的数量是一个字符串，
// 他们可能输入了任何内容，而不仅仅是数字！
//
// 目前，这个函数根本没有处理错误情况（也没有正确处理成功情况）。
// 我们想要做的是：如果我们在一个不是数字的字符串上调用 `parse` 函数，
// 该函数将返回一个 `ParseIntError`，在这种情况下，我们希望立即从函数返回该错误，
// 而不是尝试进行乘法和加法。
//
// 至少有两种方法可以实现这一点，它们都是正确的——但其中一种要短得多！
//
// Execute `rustlings hint errors2` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

use std::num::ParseIntError;

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>();
    match qty {
        Ok(qty) => Ok(qty * cost_per_item + processing_fee),
        Err(e) => Err(e),
    }
    // Ok(qty * cost_per_item + processing_fee)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().to_string(),
            "invalid digit found in string"
        );
    }
}
