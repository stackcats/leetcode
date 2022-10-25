defmodule Solution do
  @spec most_frequent_even(nums :: [integer]) :: integer
  def most_frequent_even(nums) do
    nums
    |> Enum.reduce(%{}, fn n, acc ->
      if rem(n, 2) == 0 do
        Map.update(acc, n, 1, &(&1 + 1))
      else
        acc
      end
    end)
    |> Enum.max_by(fn {k, v} -> {v, -k} end, fn -> {-1} end)
    |> elem(0)
  end
end
