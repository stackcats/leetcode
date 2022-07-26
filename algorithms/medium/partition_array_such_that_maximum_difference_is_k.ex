defmodule Solution do
  @spec partition_array(nums :: [integer], k :: integer) :: integer
  def partition_array(nums, k) do
    nums
    |> Enum.sort()
    |> then(&aux(&1, hd(&1), k, 1))
  end

  def aux([], _m, _k, ct), do: ct
  def aux([n | ns], m, k, ct) when n <= m + k, do: aux(ns, m, k, ct)
  def aux([n | ns], m, k, ct), do: aux(ns, n, k, ct + 1)
end
