// variables3.rs
// Make me compile! Execute the command `rustlings hint variables3` if you want a hint :)

// rustc --explain E0384 变量默认immutable, 如果需要改变,需要声明mut

fn main() {
    let mut x = 3;
    println!("Number {}", x);
    x = 5; // don't change this line
    println!("Number {}", x);
}
