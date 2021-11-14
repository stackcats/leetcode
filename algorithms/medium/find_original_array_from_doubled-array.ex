defmodule Solution do
  @spec find_original_array(changed :: [integer]) :: [integer]
  def find_original_array(changed) do
    map =
      Enum.reduce(changed, %{}, fn n, acc ->
        Map.update(acc, n, 1, &(&1 + 1))
      end)

    aux(0, 100_000, map, [])
  end

  defp aux(i, j, map, ans) do
    cond do
      i > j ->
        ans

      Map.get(map, i, 0) == 0 ->
        aux(i + 1, j, map, ans)

      i * 2 > j ->
        []

      not Map.has_key?(map, i * 2) ->
        []

      true ->
        case reduce(map, i, ans) do
          {_, []} ->
            []

          {map, ans} ->
            aux(i + 1, j, map, ans)
        end
    end
  end

  defp reduce(map, i, ans) do
    cond do
      map[i] == 0 ->
        {map, ans}

      map[2 * i] <= 0 ->
        {map, []}

      true ->
        map = Map.update!(map, i * 2, &(&1 - 1))
        map = Map.update!(map, i, &(&1 - 1))
        reduce(map, i, [i | ans])
    end
  end
end
