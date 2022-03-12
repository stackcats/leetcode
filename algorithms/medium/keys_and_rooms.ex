defmodule Solution do
  @spec can_visit_all_rooms(rooms :: [[integer]]) :: boolean
  def can_visit_all_rooms(rooms) do
    rooms
    |> to_map()
    |> find(0)
    |> Map.size()
    |> then(&(&1 == 0))
  end

  def to_map(rooms) do
    rooms
    |> Enum.with_index()
    |> Enum.reduce(%{}, fn {keys, i}, acc ->
      Map.put(acc, i, keys)
    end)
  end

  def find(rooms, key) do
    keys = Map.get(rooms, key, [])
    rooms = Map.delete(rooms, key)

    Enum.reduce(keys, rooms, fn k, acc ->
      find(acc, k)
    end)
  end
end
