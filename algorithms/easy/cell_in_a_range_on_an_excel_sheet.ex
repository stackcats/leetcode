defmodule Solution do
  @spec cells_in_range(s :: String.t()) :: [String.t()]
  def cells_in_range(s) do
    [a, b] = String.split(s, ":")
    [r1, c1] = String.graphemes(a)
    [r2, c2] = String.graphemes(b)
    r1 = :binary.first(r1)
    r2 = :binary.first(r2)
    c1 = String.to_integer(c1)
    c2 = String.to_integer(c2)

    for i <- r1..r2, j <- c1..c2 do
      "#{List.to_string([i])}#{j}"
    end
  end
end
