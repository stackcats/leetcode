defmodule Solution do
  @spec sequential_digits(low :: integer, high :: integer) :: [integer]
  def sequential_digits(low, high) do
    1..9
    |> Enum.reduce([], &aux(&1, &2, low, high))
    |> Enum.sort()
  end

  def aux(n, acc, low, high) do
    d = rem(n, 10)

    cond do
      n < low && d != 0 -> aux(n * 10 + d + 1, acc, low, high)
      n <= high && d != 0 -> aux(n * 10 + d + 1, [n | acc], low, high)
      true -> acc
    end
  end
end
