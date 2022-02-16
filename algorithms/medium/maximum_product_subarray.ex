defmodule Solution do
  @spec max_product(nums :: [integer]) :: integer
  def max_product([x | xs]) do
    xs
    |> Enum.reduce({x, x, x}, fn n, {ans, mi, ma} ->
      {mi, ma} = if n < 0, do: {ma, mi}, else: {mi, ma}
      mi = min(n, mi * n)
      ma = max(n, ma * n)
      {max(ans, ma), mi, ma}
    end)
    |> elem(0)
  end
end
