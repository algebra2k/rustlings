// variables6.rs
// Make me compile! Execute the command `rustlings hint variables6` if you want a hint :)

// const 和 let 一样, 也需要未编译器提供类型推断所需的上下文, 如果没有则需要指定
// const NUMBER = 3;
const NUMBER: i8 = 3;
fn main() {
    println!("Number {}", NUMBER);
}
