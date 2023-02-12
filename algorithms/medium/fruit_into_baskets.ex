defmodule Solution do
  @spec total_fruit(fruits :: [integer]) :: integer
  def total_fruit(fruits) do
    arr = :array.from_list(fruits)
    aux(arr, :array.size(arr), 0, 0, %{})
  end

  def aux(arr, last, i, j, mp) when j == last, do: j - i

  def aux(arr, last, i, j, mp) do
    mp = Map.update(mp, :array.get(j, arr), 1, &(&1 + 1))

    if map_size(mp) > 2 do
      type = :array.get(i, arr)
      mp = Map.update!(mp, type, &(&1 - 1))
      mp = if mp[type] == 0, do: Map.drop(mp, [type]), else: mp
      aux(arr, last, i + 1, j + 1, mp)
    else
      aux(arr, last, i, j + 1, mp)
    end
  end
end
