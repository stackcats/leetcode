defmodule Solution do
  @spec find_matrix(nums :: [integer]) :: [[integer]]
  def find_matrix(nums) do
    nums
    |> Enum.reduce(%{}, fn n, acc ->
      Map.update(acc, n, 1, &(&1 + 1))
    end)
    |> Enum.reduce(%{}, fn {n, ct}, acc ->
      1..ct
      |> Enum.reduce(acc, fn index, acc ->
        Map.update(acc, index, [n], &[n | &1])
      end)
    end)
    |> Map.values()
  end
end
