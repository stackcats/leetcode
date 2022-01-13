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
  @spec pair_sum(head :: ListNode.t() | nil) :: integer
  def pair_sum(head) do
    {a, b} = split(head)
    b = reverse(b)
    pair_sum(a, b)
  end

  def pair_sum(a, b), do: pair_sum(a, b, 0)

  def pair_sum(_a, nil, ans), do: ans

  def pair_sum(a, b, ans) do
    pair_sum(a.next, b.next, max(a.val + b.val, ans))
  end

  def split(head), do: split(head, head, head)

  def split(head, slow, fast) do
    if fast == nil || fast.next == nil do
      {head, slow}
    else
      split(head, slow.next, fast.next.next)
    end
  end

  def reverse(head), do: reverse(head, nil)

  def reverse(nil, reversed), do: reversed

  def reverse(head, reversed) do
    reverse(head.next, %{head | next: reversed})
  end
end
