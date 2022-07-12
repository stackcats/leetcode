# Definition for a binary tree node.
#
# defmodule TreeNode do
#   @type t :: %__MODULE__{
#           val: integer,
#           left: TreeNode.t() | nil,
#           right: TreeNode.t() | nil
#         }
#   defstruct val: 0, left: nil, right: nil
# end

defmodule Solution do
  @spec evaluate_tree(root :: TreeNode.t() | nil) :: boolean
  def evaluate_tree(%TreeNode{val: 0}), do: false
  def evaluate_tree(%TreeNode{val: 1}), do: true

  def evaluate_tree(root) do
    left = evaluate_tree(root.left)
    right = evaluate_tree(root.right)

    if root.val == 2 do
      left || right
    else
      left && right
    end
  end
end
