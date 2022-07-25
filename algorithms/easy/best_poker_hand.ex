defmodule Solution do
  @spec best_hand(ranks :: [integer], suits :: [char]) :: String.t()
  def best_hand(_ranks, [a, a, a, a, a]), do: "Flush"

  def best_hand(ranks, _suits) do
    ranks
    |> Enum.reduce(%{}, fn r, acc ->
      Map.update(acc, r, 1, &(&1 + 1))
    end)
    |> Map.values()
    |> Enum.max()
    |> then(fn v ->
      case v do
        1 -> "High Card"
        2 -> "Pair"
        _ -> "Three of a Kind"
      end
    end)
  end
end
