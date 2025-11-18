var minSubArrayLen = function (target, nums) {
  const n = nums.length;
  let ans = n + 1;
  let sum = 0; // 子数组元素和
  let left = 0; // 子数组左端点
  for (let right = 0; right < n; right++) {
    // 枚举子数组右端点
    sum += nums[right];
    while (sum - nums[left] >= target) {
      // 尽量缩小子数组长度
      sum -= nums[left];
      left++; // 左端点右移
    }
    if (sum >= target) {
      ans = Math.min(ans, right - left + 1);
    }
  }
  return ans <= n ? ans : 0;
};

// 作者：灵茶山艾府
// 链接：https://leetcode.cn/problems/minimum-size-subarray-sum/solutions/1959532/biao-ti-xia-biao-zong-suan-cuo-qing-kan-k81nh/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。
