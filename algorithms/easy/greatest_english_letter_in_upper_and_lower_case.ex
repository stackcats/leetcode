defmodule Solution do
  @spec greatest_letter(s :: String.t()) :: String.t()
  def greatest_letter(s) do
    set =
      s
      |> String.to_charlist()
      |> MapSet.new()

    ?Z..?A//-1
    |> Enum.reduce_while("", fn c, _ ->
      if MapSet.member?(set, c) && MapSet.member?(set, c + 32) do
        {:halt, List.to_string([c])}
      else
        {:cont, ""}
      end
    end)
  end
end
