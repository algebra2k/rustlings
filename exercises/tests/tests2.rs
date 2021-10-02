// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests2` for hints :)

fn reserve_le_for(nums: &mut Vec<i32>, num: i32) {
    nums.retain(|&x| x < num);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert_eq() {
        let mut nums = vec![1, 2, 3, 4, 100];
        reserve_le_for(&mut nums, 100);
        assert_eq!(vec!(1, 2, 3, 4), nums);
    }
}
