// errors1.rs
//
// This function refuses to generate text to be printed on a nametag if you pass
// it an empty string. It'd be nicer if it explained what the problem was,
// instead of just sometimes returning `None`. Thankfully, Rust has a similar
// construct to `Result` that can be used to express error conditions. Let's use
// it!
// 如果你传递一个空字符串，这个函数将拒绝生成要打印在名牌上的文本。
// 如果它能解释问题所在，而不是有时返回 `None`，那就更好了。
// 幸运的是，Rust 有一个类似于 `Result` 的结构，可以用来表示错误情况。让我们使用它吧！
//
// Execute `rustlings hint errors1` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

pub fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        // Empty names aren't allowed.
        Err("`name` was empty; it must be nonempty.".into())
    } else {
        Ok(format!("Hi! My name is {}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            // Don't change this line
            Err("`name` was empty; it must be nonempty.".into())
        );
    }
}
