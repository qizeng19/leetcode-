use std::collections::HashSet;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut set = nums1.into_iter().collect::<HashSet<_>>();
        let mut ans = Vec::new();

        for x in nums2 {
            if set.remove(&x) {
                ans.push(x);
            }
        }
        ans
    }
}

// `Vec<i32>` 本身不可能直接变成 `HashSet`，需要先把里面的元素一个个“迭代”出来，这就是 `into_iter()` 的作用：把 `Vec` 按值拿出元素，产生一个迭代器。接着调用 `collect()` 可以把“某个迭代器的所有元素”汇总成你想要的集合类型，但编译器需要知道目标类型是啥，于是就用 `::<HashSet<_>>` 这个“涡流鱼”(turbofish) 语法来指明“把这些元素收集成 `HashSet`，元素类型编译器自己推断”。`HashMap` 也一样：你先拿到一个 `(key, value)` 的迭代器，再 `collect::<HashMap<_, _>>()`。
