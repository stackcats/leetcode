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
  @spec min_camera_cover(root :: TreeNode.t() | nil) :: integer
  def min_camera_cover(root) do
    case aux(root, 0) do
      {:needed, ct} -> ct + 1
      {_, ct} -> ct
    end
  end

  def aux(nil, ct), do: {:watched, ct}

  def aux(root, ct) do
    {left, ct} = aux(root.left, ct)
    {right, ct} = aux(root.right, ct)

    cond do
      left == :needed || right == :needed -> {:placed, ct + 1}
      left == :placed || right == :placed -> {:watched, ct}
      true -> {:needed, ct}
    end
  end
end
