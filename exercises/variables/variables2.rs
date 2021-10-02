// variables2.rs
// Make me compile! Execute the command `rustlings hint variables2` if you want a hint :)

// 1. E282 说明了编译器无法从上下文环境中推断出类型时, 编译器会报错
// 2. E383 说明了未初始化的值使用时, 编译器会报错

fn main() {
    let x: i32 = 10;
    if x == 10 {
        println!("Ten!");
    } else {
        println!("Not ten!");
    }
}
