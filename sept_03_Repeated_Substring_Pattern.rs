impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        // if s == "" {
        //     return false;
        // }
        // if s.len()%2==1{ return  false}
        // // to increase window size per iteration
        // for i in 0..s.len() / 2{
        //     let mut s3: &str = &s[0..i];
        //                 print!(" {} {}", s3,s.len() / 2);
        //     let mut count_size = s.matches(&s3).count();
        //     if (count_size > 1 && i * count_size == s.len()) {
        //         print!("returining {}", s3);
        //         return true;
        //     }
        // }
        // false
        return (s.clone() + &s)[1..s.len() * 2 - 1].contains(&s);
    }
}
