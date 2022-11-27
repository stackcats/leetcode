defmodule Solution do
  @spec pivot_integer(n :: integer) :: integer
  def pivot_integer(n) do
    x = :math.sqrt((n * n + n) / 2)

    if ceil(x) == trunc(x) do
      trunc(x)
    else
      -1
    end
  end
end
