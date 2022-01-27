defmodule Solution do
  use Bitwise

  @spec count_max_or_subsets(nums :: [integer]) :: integer
  def count_max_or_subsets(nums) do
    count_max_or_subsets(nums, 0, -1, 0) |> elem(1)
  end

  def count_max_or_subsets(nums, cur, max, ans) do
    {max, ans} =
      cond do
        cur > max -> {cur, 1}
        cur == max -> {max, ans + 1}
        true -> {max, ans}
      end

    if nums == [] do
      {max, ans}
    else
      for i <- 0..(Enum.count(nums) - 1), reduce: {max, ans} do
        {max, ans} ->
          [x | xs] = Enum.drop(nums, i)
          count_max_or_subsets(xs, bor(cur, x), max, ans)
      end
    end
  end
end
