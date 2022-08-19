defmodule Solution do
  @spec num_factored_binary_trees(arr :: [integer]) :: integer
  def num_factored_binary_trees(arr) do
    arr
    |> Enum.sort()
    |> Enum.reduce(%{}, fn n, dp ->
      dp
      |> Enum.filter(fn {k, _} ->
        rem(n, k) == 0
      end)
      |> Enum.reduce(1, fn {k, v}, ct ->
        ct + Map.get(dp, div(n, k), 0) * v
      end)
      |> then(&Map.put(dp, n, &1))
    end)
    |> Map.values()
    |> Enum.sum()
    |> then(&rem(&1, 1_000_000_007))
  end
end
