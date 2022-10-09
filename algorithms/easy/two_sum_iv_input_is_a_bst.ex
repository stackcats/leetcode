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
  @spec find_target(root :: TreeNode.t() | nil, k :: integer) :: boolean
  def find_target(root, k) do
    dfs(root, MapSet.new(), k)
    |> elem(0)
  end

  def dfs(nil, set, _k), do: {false, set}

  def dfs(curr, set, k) do
    if (k - curr.val) in set do
      {true, set}
    else
      {r, set} = dfs(curr.left, MapSet.put(set, curr.val), k)

      if r do
        {r, set}
      else
        dfs(curr.right, set, k)
      end
    end
  end
end
