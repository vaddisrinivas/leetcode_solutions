impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        for i in 0..nums.len() {
            for j in 0..nums.len() {
                if i != j
                    && (nums[i] as i64 - nums[j] as i64).abs() <= t as i64
                    && ((i as i32) - (j as i32)).abs() <= k
                {
                    // println!("{} {} {} {}", nums[i], nums[j], i, j);
                    return true;
                }
            }
        }
        return false;
    }
}
