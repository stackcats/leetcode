defmodule Solution do
  @spec min_swaps(s :: String.t()) :: integer
  def min_swaps(s) do
    s
    |> String.graphemes()
    |> Enum.reduce(0, fn c, ct ->
      cond do
        c == "[" -> ct + 1
        c == "]" && ct > 0 -> ct - 1
        true -> ct
      end
    end)
    |> then(&(&1 / 2))
    |> ceil()
  end
end
