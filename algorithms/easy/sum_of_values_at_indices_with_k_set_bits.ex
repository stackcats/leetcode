defmodule Solution do
  use Bitwise

  @spec sum_indices_with_k_set_bits(nums :: [integer], k :: integer) :: integer
  def sum_indices_with_k_set_bits(nums, k) do
    nums
    |> Enum.with_index()
    |> Enum.reduce(0, fn {n, i}, acc ->
      if count_one(i, 0) == k do
        acc + n
      else
        acc
      end
    end)
  end

  def count_one(n, ct) when n == 0, do: ct

  def count_one(n, ct) do
    if (n &&& 1) == 1 do
      count_one(n >>> 1, ct + 1)
    else
      count_one(n >>> 1, ct)
    end
  end
end
