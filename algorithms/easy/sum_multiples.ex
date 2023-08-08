defmodule Solution do
  @spec sum_of_multiples(n :: integer) :: integer
  def sum_of_multiples(n) do
    1..n
    |> Enum.filter(fn n ->
      [3, 5, 7]
      |> Enum.any?(fn m -> rem(n, m) == 0 end)
    end)
    |> Enum.sum()
  end
end
