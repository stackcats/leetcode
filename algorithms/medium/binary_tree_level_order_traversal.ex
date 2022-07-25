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
  @spec level_order(root :: TreeNode.t() | nil) :: [[integer]]
  def level_order(nil), do: []

  def level_order(root) do
    aux([root], [])
  end

  def aux([], ls) do
    ls |> Enum.reverse()
  end

  def aux(rs, ls) do
    {rs, l} =
      rs
      |> Enum.reduce({[], []}, fn r, {rs, l} ->
        rs = if r.left, do: [r.left | rs], else: rs
        rs = if r.right, do: [r.right | rs], else: rs
        {rs, [r.val | l]}
      end)

    aux(Enum.reverse(rs), [Enum.reverse(l) | ls])
  end
end
