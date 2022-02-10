defmodule Solution do
  @spec minimum_sum(num :: integer) :: integer
  def minimum_sum(num) do
    [a, b, c, d] = num |> split() |> Enum.sort()
    10 * a + c + 10 * b + d
  end

  def split(num), do: split(num, [])
  def split(0, lst), do: lst

  def split(num, lst) do
    split(div(num, 10), [rem(num, 10) | lst])
  end
end
