# Definition for singly-linked list.
#
# defmodule ListNode do
#   @type t :: %__MODULE__{
#           val: integer,
#           next: ListNode.t() | nil
#         }
#   defstruct val: 0, next: nil
# end

defmodule Solution do
  @spec merge_nodes(head :: ListNode.t() | nil) :: ListNode.t() | nil
  def merge_nodes(head) do
    aux(nil, 0, head.next)
  end

  def aux(ans, _cur_val, nil), do: reverse(ans)

  def aux(ans, cur_val, %ListNode{val: val, next: next}) do
    if val == 0 do
      aux(%ListNode{val: cur_val, next: ans}, 0, next)
    else
      aux(ans, cur_val + val, next)
    end
  end

  def reverse(lst), do: reverse(lst, nil)

  def reverse(nil, lst), do: lst

  def reverse(%ListNode{val: val, next: next}, lst) do
    reverse(next, %ListNode{val: val, next: lst})
  end
end
