defmodule Solution do
  @spec arithmetic_triplets(nums :: [integer], diff :: integer) :: integer
  def arithmetic_triplets(nums, diff) do
    nums
    |> Enum.reduce({MapSet.new(), 0}, fn n, {set, ct} ->
      if MapSet.member?(set, n - diff) && MapSet.member?(set, n - diff - diff) do
        {MapSet.put(set, n), ct + 1}
      else
        {MapSet.put(set, n), ct}
      end
    end)
    |> elem(1)
  end
end
