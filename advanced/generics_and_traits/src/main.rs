// traits AddAssign and Copy guarantees the compiler that any data type used will implement AddAssign and Copy trait. (allows doing += & =)
fn sum<T: std::ops::AddAssign + Copy>(nums: &Vec<T>) -> T {
    let mut total: T = nums[0];
    for i in 1..nums.len() {
        total += nums[i];
    }
    total
}

fn main() {
    let nums: Vec<i32> = vec![1, 2, 3];
    println!("{}", sum(&nums));
}
