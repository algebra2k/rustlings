// functions2.rs
// Make me compile! Execute `rustlings hint functions2` for hints :)

fn main() {
    call_me(3);
}

// fn call_me(num:) {
// for i in 0..num {
// println!("Ring! Call number {}", i + 1);
// }
// }

fn call_me(num: i32) {
    let mut i: i32 = num;
    println!("Ring! Call number {}", i + 1);
}
