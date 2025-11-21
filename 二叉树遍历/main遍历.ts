interface TreeNode<T = number> {
    val: T;
    left: TreeNode<T> | null;
    right: TreeNode<T> | null;
}

// 先序遍历（Preorder：根 - 左 - 右）
function preorderTraversal<T>(root: TreeNode<T> | null): T[] {
    const res: T[] = []
    if (!root) return res

    const stack: TreeNode<T>[] = [root]

    while (stack.length) {
        const node = stack.pop()!
        res.push(node.val)

        if (node.right) stack.push(node.right)
        if (node.left) stack.push(node.left)
    }
    return res
}