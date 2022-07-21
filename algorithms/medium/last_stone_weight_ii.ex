defmodule Solution do
  @spec last_stone_weight_ii(stones :: [integer]) :: integer
  def last_stone_weight_ii(stones) do
    sum = Enum.sum(stones)
    target = div(sum, 2)

    for stone <- stones, j <- target..stone//-1, reduce: %{} do
      dp -> Map.put(dp, j, max(Map.get(dp, j, 0), Map.get(dp, j - stone, 0) + stone))
    end
    |> Map.get(target, 0)
    |> then(&(sum - 2 * &1))
  end
end
