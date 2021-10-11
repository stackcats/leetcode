defmodule Solution do
  @spec two_out_of_three(nums1 :: [integer], nums2 :: [integer], nums3 :: [integer]) :: [integer]
  def two_out_of_three(nums1, nums2, nums3) do
    m = nums1 |> Enum.uniq |> Enum.reduce(%{}, &update/2)
    m = nums2 |> Enum.uniq |> Enum.reduce(m, &update/2)
    m = nums3 |> Enum.uniq |> Enum.reduce(m, &update/2)
    for {k, v} <- m, v > 1, do: k
  end

  def update(k, m) do
    Map.update(m, k, 1, & &1 + 1)
  end
end
