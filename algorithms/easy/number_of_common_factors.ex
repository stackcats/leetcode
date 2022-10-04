defmodule Solution do
  @spec common_factors(a :: integer, b :: integer) :: integer
  def common_factors(a, b) do
    for i <- 1..min(a, b), reduce: 0 do
      acc -> if rem(a, i) == 0 && rem(b, i) == 0, do: acc + 1, else: acc
    end
  end
end
