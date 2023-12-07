defmodule Solution do
  @spec min_number(nums1 :: [integer], nums2 :: [integer]) :: integer
  def min_number(nums1, nums2) do
    st1 = MapSet.new(nums1)
    st2 = MapSet.new(nums2)

    MapSet.intersection(st1, st2)
    |> Enum.min(fn ->
      a = Enum.min(nums1)
      b = Enum.min(nums2)
      min(a, b) * 10 + max(a, b)
    end)
  end
end
