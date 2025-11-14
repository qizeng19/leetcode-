impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;
        let mut max_s = i32::MIN; // 窗口元素和的最大值 注意nums 中有负数，代码中的 maxS 要初始化成 −∞
        let mut s = 0; // 维护窗口元素和
        for (i, &x) in nums.iter().enumerate() {
            // 1. 进入窗口
            s += x;
            if i + 1 < k {
                // 窗口大小不足 k
                continue;
            }
            // 2. 更新答案
            max_s = max_s.max(s);
            // 3. 离开窗口
            s -= nums[i + 1 - k];
        }
        (max_s as f64) / (k as f64)
    }
}

// 作者：灵茶山艾府
// 链接：https://leetcode.cn/problems/maximum-average-subarray-i/solutions/3061219/mo-ban-ding-chang-hua-dong-chuang-kou-py-1jxk/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。
