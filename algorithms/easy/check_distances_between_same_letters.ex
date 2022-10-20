defmodule Solution do
  @spec check_distances(s :: String.t(), distance :: [integer]) :: boolean
  def check_distances(s, distance) do
    distance =
      distance
      |> Enum.with_index()
      |> Enum.reduce(%{}, fn {d, i}, acc ->
        Map.put(acc, List.to_string([i + ?a]), d)
      end)

    s
    |> String.graphemes()
    |> Enum.with_index()
    |> Enum.reduce_while({%{}, true}, fn {c, i}, {mp, ans} ->
      cond do
        mp[c] == nil -> {:cont, {Map.put(mp, c, i), ans}}
        distance[c] != i - mp[c] - 1 -> {:halt, {mp, false}}
        true -> {:cont, {mp, ans}}
      end
    end)
    |> elem(1)
  end
end
