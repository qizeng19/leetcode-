var intersection = function (nums1, nums2) {
  const st = new Set(nums1);
  const ans = [];
  for (const x of nums2) {
    if (st.delete(x)) {
      // x 在 st 中
      ans.push(x);
    }
  }
  return ans;
};

// 作者：灵茶山艾府
// 链接：https://leetcode.cn/problems/intersection-of-two-arrays/solutions/3053420/yi-ci-bian-li-pythonjavacgojsrust-by-end-jstx/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。
