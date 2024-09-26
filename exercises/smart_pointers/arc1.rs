// arc1.rs
//
// In this exercise, we are given a Vec of u32 called "numbers" with values
// ranging from 0 to 99 -- [ 0, 1, 2, ..., 98, 99 ] We would like to use this
// set of numbers within 8 different threads simultaneously. Each thread is
// going to get the sum of every eighth value, with an offset.
//
// The first thread (offset 0), will sum 0, 8, 16, ...
// The second thread (offset 1), will sum 1, 9, 17, ...
// The third thread (offset 2), will sum 2, 10, 18, ...
// ...
// The eighth thread (offset 7), will sum 7, 15, 23, ...
//
// Because we are using threads, our values need to be thread-safe.  Therefore,
// we are using Arc.  We need to make a change in each of the two TODOs.
//
// Make this code compile by filling in a value for `shared_numbers` where the
// first TODO comment is, and create an initial binding for `child_numbers`
// where the second TODO comment is. Try not to create any copies of the
// `numbers` Vec!
// 在这个练习中，我们有一个名为 "numbers" 的 Vec<u32>，其值范围从 0 到 99 -- [ 0, 1, 2, ..., 98, 99 ]
// 我们希望在 8 个不同的线程中同时使用这组数字。每个线程将获取每第八个值的总和，并有一个偏移量。
//
// 第一个线程（偏移量 0）将求和 0, 8, 16, ...
// 第二个线程（偏移量 1）将求和 1, 9, 17, ...
// 第三个线程（偏移量 2）将求和 2, 10, 18, ...
// ...
// 第八个线程（偏移量 7）将求和 7, 15, 23, ...
//
// 因为我们使用的是线程，所以我们的值需要是线程安全的。因此，我们使用 Arc。
// 我们需要在两个 TODO 处进行更改。
//
// 通过在第一个 TODO 注释处填写 `shared_numbers` 的值，并在第二个 TODO 注释处创建 `child_numbers` 的初始绑定，使此代码可以编译。
// 尽量不要创建 `numbers` Vec 的任何副本！
//
// Execute `rustlings hint arc1` or use the `hint` watch subcommand for a hint.

// I AM DONE

#![forbid(unused_imports)] // Do not change this, (or the next) line.
use std::sync::Arc;
use std::thread;

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = Arc::new(numbers);
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        let child_numbers = shared_numbers.clone();
        joinhandles.push(thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}
