defmodule Solution do
  @spec left_right_difference(nums :: [integer]) :: [integer]
  def left_right_difference(nums) do
    right = Enum.sum(nums)

    nums
    |> Enum.reduce({[], 0, right}, fn n, {acc, left, right} ->
      {[abs(right - left - n) | acc], left + n, right - n}
    end)
    |> elem(0)
    |> Enum.reverse()
  end
end
