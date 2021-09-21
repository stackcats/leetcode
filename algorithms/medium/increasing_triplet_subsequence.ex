defmodule Solution do
  @spec increasing_triplet(nums :: [integer]) :: boolean
  def increasing_triplet(nums) do
    low = 2147483647
    mid = 2147483647
    r(low, mid, nums)
  end
  
  def r(low, mid, []), do: false
  def r(low, mid, [head|rest]) do
    cond do
      low > head -> r(head, mid, rest)
      head > mid -> true
      head > low and head < mid -> r(low, head, rest)
      true -> r(low, mid, rest)
    end
  end
end
