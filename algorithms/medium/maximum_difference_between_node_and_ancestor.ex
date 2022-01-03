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
  @spec max_ancestor_diff(root :: TreeNode.t() | nil) :: integer
  def max_ancestor_diff(root) do
    aux(root, root.val, root.val)
  end

  def aux(nil, mi, ma), do: ma - mi

  def aux(root, mi, ma) do
    mi = min(mi, root.val)
    ma = max(ma, root.val)
    left = aux(root.left, mi, ma)
    right = aux(root.right, mi, ma)
    max(left, right)
  end
end
