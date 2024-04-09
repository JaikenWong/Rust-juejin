impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut candidate:i32 = -1;

        let mut count = 0;

        for num in &nums {
            if count == 0 {
                candidate = *num;
            }
            if *num == candidate {
                count += 1;
            } else {
                count -= 1;
            }
        }

        count = 0;
        for num in &nums {
            if *num == candidate {
                count += 1;
            }
        }
        if count * 2 > nums.len() {
            return candidate;
        }
        return -1;
    }
}

struct Solution {

}

fn main() {
    let nums: Vec<i32> = vec![1, 2, 5, 9, 5, 9, 5, 5, 5];
    let solution = Solution::majority_element(nums);

    println!("result is {}", solution);
}
