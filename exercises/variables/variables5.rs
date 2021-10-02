// variables5.rs
// Make me compile! Execute the command `rustlings hint variables5` if you want a hint :)

// rustc --explain E0308
fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    // 变量可以被覆盖, 覆盖可以是其他类型
    // number = 3
    let mut number = 3;
    println!("Number plus two is : {}", number + 2);
}
