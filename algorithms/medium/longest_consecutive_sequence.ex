defmodule Solution do
  @spec longest_consecutive(nums :: [integer]) :: integer
  def longest_consecutive(nums) do
    aux(nums, %{}, 0)
  end

  defp aux([], _map, ans), do: ans

  defp aux([x | xs], map, ans) do
    if Map.has_key?(map, x) do
      aux(xs, map, ans)
    else
      left = Map.get(map, x - 1, 0)
      right = Map.get(map, x + 1, 0)
      len = left + right + 1
      new_map = map |> Map.put(x, len) |> Map.put(x - left, len) |> Map.put(x + right, len)
      aux(xs, new_map, max(ans, len))
    end
  end
end
