defmodule Solution do
  @spec find_non_min_or_max(nums :: [integer]) :: integer
  def find_non_min_or_max(nums) do
    nums = Enum.take(nums, 3)

    if length(nums) < 3 do
      -1
    else
      [_, a, _] = Enum.sort(nums)
      a
    end
  end
end
