use std::collections::HashSet;

impl Solution {
    pub fn largest_time_from_digits(a: Vec<i32>) -> String {
        if a.iter().all(|&i| i == 0) {
            // if all zeros
            return String::from("00:00");
        }
        let mut old_val = 0;
        if a.iter().any(|&i| i > 9 || i < 0) {
            // if any value greater than 9 or less than 0
            return String::from("");
        }
        if a.len() != 4 {
            // if array length is not 4
            return String::from("");
        }
        if a.iter().any(|&i| i <= 2 && i >= 0) {
            // if atleast 1 value for 24 hr clock
            let mut res: Vec<i32> = Vec::new();
            permutate(&mut a.to_vec(), &mut res, 0, a.len());
            let set: HashSet<_> = res.drain(..).collect(); // collect duplicates
            res.extend(set.into_iter());
            for i in res.iter() {
                if *i > 2360 || *i % 100 >= 60 {
                    continue;
                } else {
                    if *i > old_val {
                        old_val = *i;
                    }
                }
            }
        } else {
            return String::from("");
        }
        if old_val == 0 {
            return String::from("");
        }

        let mut dts = num_digits(old_val);
        let mut n = vec![0, 0, 0, 0];
        for i in 0..dts.len() {
            let mut k = n.len() - dts.len() + i;
            n[k] = dts[i];
        }
        return String::from(format!("{}{}:{}{}", n[0], n[1], n[2], n[3]));
    }
}
pub fn num_digits(num: i32) -> Vec<i32> {
    let mul = if num < 0 { -1 } else { 1 };
    num.to_string()
        .chars()
        .filter_map(|x| x.to_digit(10))
        .map(|x| (x as i32) * mul)
        .collect()
}
pub fn permutate(a: &mut Vec<i32>, res: &mut Vec<i32>, cur: usize, size: usize) {
    res.push(make_number(a.to_vec()));
    if cur == size {
        return;
    }
    for i in cur..size {
        a.swap(cur, i);
        permutate(a, res, cur + 1, size);
        a.swap(cur, i);
    }
}
pub fn make_number(a: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut i = a.len() as u32;
    let mut base: i32 = 10;
    for j in a.iter() {
        res += j * base.pow(i - 1);
        i -= 1
    }
    res
}
