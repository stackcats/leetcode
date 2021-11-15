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
  @spec find_frequent_tree_sum(root :: TreeNode.t() | nil) :: [integer]
  def find_frequent_tree_sum(root) do
    {_, map} = aux(root, Map.new())
    most_frequent = map |> Map.values() |> Enum.max()
    map |> Enum.filter(fn {k, v} -> v == most_frequent end) |> Enum.map(fn {k, _} -> k end)
  end

  def aux(nil, map), do: {0, map}

  def aux(root, map) do
    {left_sum, map} = aux(root.left, map)
    {right_sum, map} = aux(root.right, map)
    sum = left_sum + right_sum + root.val
    {sum, Map.update(map, sum, 1, &(&1 + 1))}
  end
end
