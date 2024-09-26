// box1.rs
//
// At compile time, Rust needs to know how much space a type takes up. This
// becomes problematic for recursive types, where a value can have as part of
// itself another value of the same type. To get around the issue, we can use a
// `Box` - a smart pointer used to store data on the heap, which also allows us
// to wrap a recursive type.
//
// The recursive type we're implementing in this exercise is the `cons list` - a
// data structure frequently found in functional programming languages. Each
// item in a cons list contains two elements: the value of the current item and
// the next item. The last item is a value called `Nil`.
//
// Step 1: use a `Box` in the enum definition to make the code compile
// Step 2: create both empty and non-empty cons lists by replacing `todo!()`
// 在编译时，Rust 需要知道一个类型占用多少空间。
// 这对于递归类型来说会变得很麻烦，因为一个值可以包含另一个相同类型的值。
// 为了解决这个问题，我们可以使用 `Box` - 一种用于在堆上存储数据的智能指针，
// 它还允许我们包装递归类型。
//
// 我们在这个练习中实现的递归类型是 `cons 列表` - 一种在函数式编程语言中经常出现的数据结构。
// cons 列表中的每个项包含两个元素：当前项的值和下一个项。最后一项是一个名为 `Nil` 的值。
//
// 第一步：在枚举定义中使用 `Box` 使代码可以编译
// 第二步：通过替换 `todo!()` 创建空的和非空的 cons 列表
//
// Note: the tests should not be changed
//
// Execute `rustlings hint box1` or use the `hint` watch subcommand for a hint.

// I AM DONE

#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

pub fn create_empty_list() -> List {
    List::Nil
}

pub fn create_non_empty_list() -> List {
    List::Cons(1, Box::new(List::Nil))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }
}
