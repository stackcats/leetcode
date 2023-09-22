defmodule Solution do
  @spec remove_stars(s :: String.t()) :: String.t()
  def remove_stars(s) do
    s
    |> String.graphemes()
    |> Enum.reduce([], fn c, acc ->
      cond do
        c == "*" && acc != [] -> tl(acc)
        c == "*" -> acc
        true -> [c | acc]
      end
    end)
    |> Enum.join("")
    |> String.reverse()
  end
end
