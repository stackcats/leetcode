defmodule Solution do
  @spec number_of_cuts(n :: integer) :: integer
  def number_of_cuts(1), do: 0

  def number_of_cuts(n) do
    if rem(n, 2) == 0, do: div(n, 2), else: n
  end
end
