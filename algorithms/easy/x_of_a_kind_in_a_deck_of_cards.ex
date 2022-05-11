defmodule Solution do
  @spec has_groups_size_x(deck :: [integer]) :: boolean
  def has_groups_size_x([_]), do: false

  def has_groups_size_x(deck) do
    deck
    |> Enum.reduce(%{}, fn d, acc -> Map.update(acc, d, 1, &(&1 + 1)) end)
    |> Map.values()
    |> Enum.reduce(&Integer.gcd/2)
    |> then(&(&1 > 1))
  end
end
