defmodule Solution do
  @spec distinct_difference_array(nums :: [integer]) :: [integer]
  def distinct_difference_array(nums) do
    mp = nums |> Enum.reduce(%{}, fn n, acc -> Map.update(acc, n, 1, &(&1 + 1)) end)

    nums
    |> Enum.reduce({[], %MapSet{}, mp}, fn n, {acc, st, mp} ->
      mp = delete(mp, n)
      st = MapSet.put(st, n)
      acc = [MapSet.size(st) - map_size(mp) | acc]
      {acc, st, mp}
    end)
    |> elem(0)
    |> Enum.reverse()
  end

  defp delete(mp, k) do
    mp = Map.update!(mp, k, &(&1 - 1))

    if mp[k] == 0 do
      Map.delete(mp, k)
    else
      mp
    end
  end
end
