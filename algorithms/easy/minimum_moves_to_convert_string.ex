defmodule Solution do
  @spec minimum_moves(s :: String.t()) :: integer
  def minimum_moves(s) do
    String.graphemes(s) |> helper(0)
  end

  def helper([], _ct), do: 0
  def helper(["O" | xs], ct), do: helper(xs, max(ct - 1, 0))
  def helper([_ | xs], 0), do: 1 + helper(xs, 2)
  def helper([_ | xs], ct), do: helper(xs, ct - 1)
end
