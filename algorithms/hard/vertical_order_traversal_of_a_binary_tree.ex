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
  @spec vertical_traversal(root :: TreeNode.t() | nil) :: [[integer]]
  def vertical_traversal(root) do
    map = aux(%{}, root, 0, 0)

    map
    |> Map.keys()
    |> Enum.sort()
    |> Enum.map(fn k ->
      map[k]
      |> Enum.sort()
      |> Enum.map(fn {_r, v} -> v end)
    end)
  end

  def aux(map, nil, _r, _c), do: map

  def aux(map, root, r, c) do
    map
    |> Map.update(c, [{r, root.val}], &[{r, root.val} | &1])
    |> aux(root.left, r + 1, c - 1)
    |> aux(root.right, r + 1, c + 1)
  end
end
