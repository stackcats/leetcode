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
  @spec insert_into_max_tree(root :: TreeNode.t() | nil, val :: integer) :: TreeNode.t() | nil
  def insert_into_max_tree(nil, val), do: %TreeNode{val: val}

  def insert_into_max_tree(%TreeNode{val: root_val, left: left, right: right} = root, val) do
    if root_val > val do
      %{root | right: insert_into_max_tree(right, val)}
    else
      %TreeNode{val: val, left: root}
    end
  end
end
