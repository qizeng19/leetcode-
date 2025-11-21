interface TreeNode<T = number> {
    val: T;
    left: TreeNode<T> | null;
    right: TreeNode<T> | null;
}

// 先序遍历（Preorder：根 - 左 - 右）
function preorderTraversal<T>(root: TreeNode<T> | null) {
    const res: T[] = []

    function dfs(node: TreeNode<T> | null) {
        if (!node) return
        res.push(node.val)
        dfs(node.left)
        dfs(node.right)
    }
    dfs(root)
    return res
}
// 中序遍历（Inorder：左 - 根 - 右）
function inorderTraversal<T>(root: TreeNode<T> | null): T[] {
    const res: T[] = [];

    function dfs(node: TreeNode<T> | null) {
        if (!node) return;
        dfs(node.left);       // 左
        res.push(node.val);   // 根
        dfs(node.right);      // 右
    }

    dfs(root);
    return res;
}

// 后序遍历（Postorder：左 - 右 - 根）
function postorderTraversal<T>(root: TreeNode<T> | null): T[] {
    const res: T[] = [];

    function dfs(node: TreeNode<T> | null) {
        if (!node) return;
        dfs(node.left);       // 左
        dfs(node.right);      // 右
        res.push(node.val);   // 根
    }

    dfs(root);
    return res;
}
