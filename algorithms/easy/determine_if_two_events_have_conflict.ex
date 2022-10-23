defmodule Solution do
  @spec have_conflict(event1 :: [String.t()], event2 :: [String.t()]) :: boolean
  def have_conflict(event1, event2) do
    [start1, end1] = event1
    [start2, end2] = event2
    (start2 >= start1 && start2 <= end1) || (start1 >= start2 && start1 <= end2)
  end
end
