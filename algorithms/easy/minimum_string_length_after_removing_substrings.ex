defmodule Solution do
  @spec min_length(s :: String.t()) :: integer
  def min_length(s) do
    s
    |> String.graphemes()
    |> Enum.reduce([], fn c, st ->
      case {st, c} do
        {[], _} -> [c]
        {["A" | xs], "B"} -> xs
        {["C" | xs], "D"} -> xs
        _ -> [c | st]
      end
    end)
    |> length()
  end
end
