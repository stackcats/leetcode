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
  @spec count_dominant_nodes(root :: TreeNode.t() | nil) :: integer
  def count_dominant_nodes(root), do: aux(root) |> elem(1)
  def aux(nil), do: {0, 0}

  def aux(%TreeNode{val: val, left: left, right: right}) do
    {max_left, ct_left} = aux(left)
    {max_right, ct_right} = aux(right)

    if val >= max_left && val >= max_right do
      {val, ct_left + ct_right + 1}
    else
      {max(max_left, max_right), ct_left + ct_right}
    end
  end
end
