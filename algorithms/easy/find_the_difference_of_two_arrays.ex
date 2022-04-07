defmodule Solution do
  @spec find_difference(nums1 :: [integer], nums2 :: [integer]) :: [[integer]]
  def find_difference(nums1, nums2) do
    s1 = MapSet.new(nums1)
    s2 = MapSet.new(nums2)
    [MapSet.difference(s1, s2) |> Enum.to_list(), MapSet.difference(s2, s1) |> Enum.to_list()]
  end
end
