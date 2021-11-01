defmodule Solution do
  @spec smallest_equal(nums :: [integer]) :: integer
  def smallest_equal(nums) do
    r = nums |> Enum.with_index() |> Enum.find(fn {n, i} -> rem(i, 10) == n end)
    if r == nil, do: -1, else: elem(r, 1)
  end
end
