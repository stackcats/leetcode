defmodule Solution do
  @spec merge_similar_items(items1 :: [[integer]], items2 :: [[integer]]) :: [[integer]]
  def merge_similar_items(items1, items2) do
    map =
      items1
      |> Enum.reduce(%{}, fn [v, w], acc ->
        Map.put(acc, v, w)
      end)

    items2
    |> Enum.reduce(map, fn [v, w], acc ->
      Map.update(acc, v, w, &(&1 + w))
    end)
    |> Enum.map(fn {v, w} -> [v, w] end)
    |> Enum.sort()
  end
end
