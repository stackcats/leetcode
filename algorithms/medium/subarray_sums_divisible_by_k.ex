defmodule Solution do
  @spec subarrays_div_by_k(nums :: [integer], k :: integer) :: integer
  def subarrays_div_by_k(nums, k) do
    nums
    |> Enum.reduce({%{0 => 1}, 0, 0}, fn n, {counter, ans, prefix_sum} ->
      prefix_sum = prefix_sum + n
      m = rem(rem(prefix_sum, k) + k, k)
      ans = ans + Map.get(counter, m, 0)
      {Map.update(counter, m, 1, &(&1 + 1)), ans, prefix_sum}
    end)
    |> elem(1)
  end
end
