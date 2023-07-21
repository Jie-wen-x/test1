// File: build_tree.go
// Created Time: 2023-07-20
// Author: hongyun-robot (1836017030@qq.com)

package chapter_divide_and_conquer

import . "github.com/krahets/hello-algo/pkg"

/* 构建二叉树：分治 */
func bfs(preorder, inorder []int, hmap map[int]int, i, l, r int) *TreeNode {
	// 子树区间为空时终止
	if r-l < 0 {
		return nil
	}
	// 初始化根节点
	root := NewTreeNode(preorder[i])
	// 查询 m ，从而划分左右子树
	m := hmap[preorder[i]]
	// 子问题：构建左子树
	root.Left = bfs(preorder, inorder, hmap, i+1, l, m-1)
	// 子问题：构建右子树
	root.Right = bfs(preorder, inorder, hmap, i+1+m-l, m+1, r)
	// 返回根节点
	return root
}

/* 构建二叉树 */
func buildTree(preorder, inorder []int) *TreeNode {
	// 初始化哈希表，存储 inorder 元素到索引的映射
	hmap := make(map[int]int, len(inorder))
	for i := 0; i < len(inorder); i++ {
		hmap[inorder[i]] = i
	}

	root := bfs(preorder, inorder, hmap, 0, 0, len(inorder)-1)
	return root
}
