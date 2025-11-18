struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let b = s.as_bytes();
        print!("{:?}", b);
        // 为了按“字符下标”滑窗，先收集为 Vec<char>
        let chars: Vec<char> = s.chars().collect();
        let mut cnt: HashMap<char, usize> = HashMap::new();

        let (mut left, mut ans) = (0usize, 0usize);

        for right in 0..chars.len() {
            let c = chars[right];
            *cnt.entry(c).or_insert(0) += 1;

            // 如果窗口内 c 的计数 > 1，就不断收缩左边界
            while *cnt.get(&c).unwrap() > 1 {
                let lc = chars[left];
                if let Some(v) = cnt.get_mut(&lc) {
                    *v -= 1;
                    if *v == 0 {
                        cnt.remove(&lc); // 可选：计数为 0 时移除
                    }
                }
                left += 1;
            }

            ans = ans.max(right - left + 1);
        }

        ans as i32
    }
}

fn main() {
    let s = String::from("abcabcbb");
    println!("{}", Solution::length_of_longest_substring(s));
}
