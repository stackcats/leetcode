defmodule Solution do
  @spec repeated_character(s :: String.t()) :: char
  def repeated_character(s) do
    s |> String.to_charlist() |> aux(MapSet.new())
  end

  def aux([c | cs], set) do
    if MapSet.member?(set, c) do
      c
    else
      aux(cs, MapSet.put(set, c))
    end
  end
end
