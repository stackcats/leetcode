# Definition for singly-linked list.
#
# defmodule ListNode do
#   @type t :: %__MODULE__{
#           val: integer,
#           next: ListNode.t() | nil
#         }
#   defstruct val: 0, next: nil
# end

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
  @spec is_sub_path(head :: ListNode.t | nil, root :: TreeNode.t | nil) :: boolean
  def is_sub_path(head, root) do
    dfs(head, root)
  end

  def dfs(_, nil), do: false

  def dfs(head, root) do
    if contains(head, root) do
      true
    else
      dfs(head, root.left) or dfs(head, root.right)
    end
  end

  def contains(nil, _), do: true
  
  def contains(_, nil), do: false
  
  def contains(head, root) do
    if head.val == root.val do
      contains(head.next, root.left) or contains(head.next, root.right)
    else
      false
    end
  end
end
