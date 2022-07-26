defmodule Solution do
  use Bitwise
  @spec maximum_xor(nums :: [integer]) :: integer
  def maximum_xor(nums) do
    Enum.reduce(nums, &bor/2)
  end
end
