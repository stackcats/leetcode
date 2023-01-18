defmodule Solution do
  @spec max_subarray_sum_circular(nums :: [integer]) :: integer
  def max_subarray_sum_circular([n | ns] = nums) do
    ns
    |> Enum.reduce({n, n, n, n}, fn x, {ma_sum, ma, mi_sum, mi} ->
      ma = max(x, ma + x)
      ma_sum = max(ma_sum, ma)
      mi = min(x, mi + x)
      mi_sum = min(mi_sum, mi)
      {ma_sum, ma, mi_sum, mi}
    end)
    |> then(fn {ma_sum, ma, mi_sum, mi} ->
      if ma_sum >= 0 do
        max(ma_sum, Enum.sum(nums) - mi_sum)
      else
        ma_sum
      end
    end)
  end
end
