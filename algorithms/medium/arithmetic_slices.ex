defmodule Solution do
  @spec number_of_arithmetic_slices(nums :: [integer]) :: integer
  def number_of_arithmetic_slices(nums) do
    nums
    |> Enum.with_index()
    |> aux(0, 1, 0)
  end

  def aux([], left, right, ans), do: ans

  def aux([_], left, right, ans), do: ans

  def aux([_, _], left, right, ans), do: ans + ct(left, right)

  def aux([{a, i}, {b, j}, {c, k} | rest], left, right, ans) do
    if a - b == b - c do
      aux([{b, j}, {c, k} | rest], left, k, ans)
    else
      aux([{b, j}, {c, k} | rest], j, right, ans + ct(left, right))
    end
  end

  def ct(left, right) when right < left + 2, do: 0

  def ct(left, right) do
    n = right - left - 1
    div((1 + n) * n, 2)
  end
end
