defmodule Solution do
  use Bitwise

  @spec maximum_strong_pair_xor(nums :: [integer]) :: integer
  def maximum_strong_pair_xor(nums) do
    for x <- nums, y <- nums, x > y, abs(x-y) <= min(x,y), reduce: 0 do
      acc -> max(acc, x ^^^ y)
      end
  end
end
