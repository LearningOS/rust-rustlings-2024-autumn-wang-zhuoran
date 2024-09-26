// using_as.rs
//
// Type casting in Rust is done via the usage of the `as` operator. Please note
// that the `as` operator is not only used when type casting. It also helps with
// renaming imports.
//
// The goal is to make sure that the division does not fail to compile and
// returns the proper type.
// 在 Rust 中，类型转换是通过使用 `as` 操作符完成的。请注意，`as` 操作符不仅用于类型转换。
// 它还可以帮助重命名导入项。
//
// 目标是确保除法操作不会编译失败，并返回正确的类型。
//
// Execute `rustlings hint using_as` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

fn average(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();
    total / values.len() as f64
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }
}
