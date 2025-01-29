defmodule Solution do
  @spec longest_zig_zag(root :: TreeNode.t() | nil) :: integer
  def longest_zig_zag(root) do
    max(aux(root.left, :left, 0), aux(root.right, :right, 0))
  end

  def aux(nil, _, len), do: len

  def aux(node, :left, len) do
    max(aux(node.left, :left, 0), aux(node.right, :right, len + 1))
  end

  def aux(node, :right, len) do
    max(aux(node.left, :left, len + 1), aux(node.right, :right, 0))
  end
end
