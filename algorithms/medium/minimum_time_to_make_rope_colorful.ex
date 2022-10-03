defmodule Solution do
  @spec min_cost(colors :: String.t(), needed_time :: [integer]) :: integer
  def min_cost(colors, needed_time) do
    colors
    |> String.graphemes()
    |> Enum.zip(needed_time)
    |> Enum.reduce({"", 0, 0, 0}, fn {c, n}, {prev, ans, sum, ma} ->
      if c != prev do
        {c, ans + sum - ma, n, n}
      else
        {c, ans, sum + n, max(ma, n)}
      end
    end)
    |> then(fn {_, ans, sum, ma} ->
      ans + sum - ma
    end)
  end
end
