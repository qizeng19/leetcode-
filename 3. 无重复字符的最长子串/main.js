var lengthOfLongestSubstring = function (s) {
  let ans = 0;
  let left = 0;
  const cnt = new Map(); // 维护从下标 left 到下标 right 的字符
  for (let right = 0; right < s.length; right++) {
    const c = s[right];
    cnt.set(c, (cnt.get(c) ?? 0) + 1);
    while (cnt.get(c) > 1) {
      // 窗口内有重复字母
      cnt.set(s[left], cnt.get(s[left]) - 1); // 移除窗口左端点字母
      left++; // 缩小窗口
    }
    ans = Math.max(ans, right - left + 1); // 更新窗口长度最大值
  }
  return ans;
};

// 作者：灵茶山艾府
// 链接：https://leetcode.cn/problems/longest-substring-without-repeating-characters/solutions/1959540/xia-biao-zong-suan-cuo-qing-kan-zhe-by-e-iaks/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。
