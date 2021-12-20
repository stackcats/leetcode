defmodule Solution do
  @spec max_subsequence(nums :: [integer], k :: integer) :: [integer]
  def max_subsequence(nums, k) do
    nums
    |> Enum.with_index()
    |> Enum.sort(fn {a, _}, {b, _} -> a >= b end)
    |> Enum.take(k)
    |> Enum.sort(fn {_, i}, {_, j} -> i <= j end)
    |> Enum.map(fn {n, _} -> n end)
  end
end
