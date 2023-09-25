defmodule Solution do
  @spec merge_arrays(nums1 :: [[integer]], nums2 :: [[integer]]) :: [[integer]]
  def merge_arrays([], nums2) do
    nums2
  end

  def merge_arrays(nums1, []) do
    nums1
  end

  def merge_arrays([[id1, v1] | rs1] = nums1, [[id2, v2] | rs2] = nums2) do
    cond do
      id1 == id2 -> [[id1, v1 + v2] | merge_arrays(rs1, rs2)]
      id1 < id2 -> [[id1, v1] | merge_arrays(rs1, nums2)]
      true -> [[id2, v2] | merge_arrays(nums1, rs2)]
    end
  end
end
