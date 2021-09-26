defmodule Solution do
  @spec find_target_sum_ways(nums :: [integer], target :: integer) :: integer
  def find_target_sum_ways(nums, target) do
    nums
    |> Enum.reduce(%{0 => 1}, &loop_mem/2)
    |> Map.get(target, 0)
  end

  def loop_mem(n, mem) do
    mem
      |> Map.to_list
      |> Enum.reduce(%{}, fn {k, v}, acc ->
        acc = Map.put(acc, k + n, Map.get(acc, k + n, 0) + v) 
        Map.put(acc, k - n, Map.get(acc, k - n, 0) + v)
      end)
  end
end
