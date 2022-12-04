defmodule Solution do
  @spec minimum_average_difference(nums :: [integer]) :: integer
  def minimum_average_difference(nums) do
    right = Enum.sum(nums)
    len = length(nums)

    nums
    |> Enum.reduce({0, 0, right, 0, right}, fn n, {ans, i, max_avg, left, right} ->
      left = left + n
      right = right - n
      left_part = div(left, i + 1)
      right_part = if i == len - 1, do: 0, else: div(right, len - i - 1)
      avg = abs(left_part - right_part)

      if max_avg > avg do
        {i, i + 1, avg, left, right}
      else
        {ans, i + 1, max_avg, left, right}
      end
    end)
    |> elem(0)
  end
end
