impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        let mut ans = 0;
        let mut s = 0;
        let k = k as usize;
        let threshold = threshold as usize;

        for (i, &x) in arr.iter().enumerate() {
            s += x;

            if i + 1 < k {
                continue;
            }

            if (s as usize) >= k * threshold {
                ans += 1;
            }

            s -= arr[i - k + 1];
        }

        ans
    }
}
impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        let mut ans = 0;
        let mut s = 0; // 维护窗口元素和
        for (i, &x) in arr.iter().enumerate() {
            // 1. 进入窗口
            s += x;
            if i < (k as usize) - 1 {
                // 窗口大小不足 k
                continue;
            }
            // 2. 更新答案
            if s >= k * threshold {
                ans += 1;
            }
            // 3. 离开窗口
            s -= arr[i - (k as usize) + 1];
        }
        ans
    }
}

// 作者：灵茶山艾府
// 链接：https://leetcode.cn/problems/number-of-sub-arrays-of-size-k-and-average-greater-than-or-equal-to-threshold/solutions/3061222/mo-ban-ding-chang-hua-dong-chuang-kou-py-85sh/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。
