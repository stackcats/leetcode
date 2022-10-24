defmodule Solution do
  @spec max_length(arr :: [String.t()]) :: integer
  def max_length(arr) do
    arr
    |> Enum.map(&String.graphemes/1)
    |> Enum.filter(fn w -> Enum.uniq(w) == w end)
    |> dfs(MapSet.new(), 0)
  end

  def dfs([], _, ans), do: ans

  def dfs([w | ws], set, ans) do
    w = MapSet.new(w)

    if MapSet.disjoint?(w, set) do
      new_set = MapSet.union(set, w)

      dfs(ws, new_set, max(ans, MapSet.size(new_set)))
      |> max(dfs(ws, set, ans))
    else
      dfs(ws, set, ans)
    end
  end
end
