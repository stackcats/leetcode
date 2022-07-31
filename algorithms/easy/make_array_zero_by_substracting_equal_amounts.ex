defmodule Solution do
  @spec minimum_operations(nums :: [integer]) :: integer
  def minimum_operations(nums) do
    nums
    |> Enum.reduce(MapSet.new(), &MapSet.put(&2, &1))
    |> MapSet.delete(0)
    |> MapSet.size()
  end
end
