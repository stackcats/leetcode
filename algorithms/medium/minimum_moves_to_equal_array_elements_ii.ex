defmodule Solution do
  @spec min_moves2(nums :: [integer]) :: integer
  def min_moves2(nums) do
    nums
    |> Enum.sort()
    |> then(fn lst ->
      median = lst |> length() |> div(2) |> then(&Enum.at(lst, &1))

      lst
      |> Enum.reduce(0, fn n, acc ->
        acc + abs(n - median)
      end)
    end)
  end
end
