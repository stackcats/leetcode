defmodule Solution do
  @spec get_sum(a :: integer, b :: integer) :: integer
  def get_sum(a, b) do
    (:math.pow(10, a) * :math.pow(10, b)) |> :math.log10() |> trunc
  end
end
