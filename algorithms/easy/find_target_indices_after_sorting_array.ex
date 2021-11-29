defmodule Solution do
  @spec target_indices(nums :: [integer], target :: integer) :: [integer]
  def target_indices(nums, target) do
    nums
    |> Enum.sort()
    |> Enum.with_index()
    |> Enum.flat_map(fn {n, i} -> if n == target, do: [i], else: [] end)
  end
end
