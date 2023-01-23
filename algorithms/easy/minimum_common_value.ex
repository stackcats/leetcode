defmodule Solution do
  @spec get_common(nums1 :: [integer], nums2 :: [integer]) :: integer
  def get_common([], _), do: -1
  def get_common(_, []), do: -1

  def get_common([n | ns], [m | ms]) do
    cond do
      n == m -> n
      n < m -> get_common(ns, [m | ms])
      true -> get_common([n | ns], ms)
    end
  end
end
