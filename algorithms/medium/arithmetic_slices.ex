defmodule Solution do
  @spec number_of_arithmetic_slices(nums :: [integer]) :: integer
  def number_of_arithmetic_slices(nums) do
    nums
    |> Enum.with_index()
    |> aux(0, 0)
  end

  def aux([], _ct, ans), do: ans

  def aux([_], _ct, ans), do: ans

  def aux([_, _], _ct, ans), do: ans

  def aux([{a, i}, {b, j}, {c, k} | rest], ct, ans) do
    if a - b == b - c do
      aux([{b, j}, {c, k} | rest], ct + 1, ans + ct + 1)
    else
      aux([{b, j}, {c, k} | rest], 0, ans)
    end
  end
end
