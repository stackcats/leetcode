defmodule Solution do
  @spec find_intersection_values(nums1 :: [integer], nums2 :: [integer]) :: [integer]
  def find_intersection_values(nums1, nums2) do
    [count(nums1, nums2), count(nums2, nums1)]
  end

  def count(nums1, nums2) do
    nums2 = MapSet.new(nums2)

    nums1
    |> Enum.count(&MapSet.member?(nums2, &1))
  end
end

