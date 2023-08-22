defmodule Solution do
  @spec sum_of_squares(nums :: [integer]) :: integer
  def sum_of_squares(nums) do
    len = Enum.count(nums)

    nums
    |> Enum.with_index()
    |> Enum.reduce(0, fn {n, i}, acc ->
      if rem(len, i + 1) == 0 do
        acc + n * n
      else
        acc
      end
    end)
  end
end
