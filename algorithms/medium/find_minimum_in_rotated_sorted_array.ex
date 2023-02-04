defmodule Solution do
  @spec find_min(nums :: [integer]) :: integer
  def find_min(nums) do
    nums =
      nums
      |> Enum.with_index()
      |> Enum.reduce(%{}, fn {n, i}, acc ->
        Map.put(acc, i, n)
      end)

    {l, r} = {0, map_size(nums) - 1}

    if nums[l] <= nums[r] do
      nums[l]
    else
      aux(nums, l, r)
    end
  end

  def aux(nums, l, r) when l > r, do: nums[l]

  def aux(nums, l, r) do
    m = l + div(r - l, 2)

    if nums[m] >= nums[0] do
      aux(nums, m + 1, r)
    else
      aux(nums, l, m - 1)
    end
  end
end
