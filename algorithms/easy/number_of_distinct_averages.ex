defmodule Solution do
  @spec distinct_averages(nums :: [integer]) :: integer
  def distinct_averages(nums) do
    arr = nums |> Enum.sort() |> :array.from_list()
    len = length(nums)

    aux(arr, 0, length(nums) - 1, MapSet.new())
  end

  def aux(arr, i, j, set) when i >= j, do: MapSet.size(set)

  def aux(arr, i, j, set) do
    sum = :array.get(i, arr) + :array.get(j, arr)
    aux(arr, i + 1, j - 1, MapSet.put(set, sum))
  end
end
