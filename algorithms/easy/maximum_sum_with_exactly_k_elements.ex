defmodule Solution do
  @spec maximize_sum(nums :: [integer], k :: integer) :: integer
  def maximize_sum(nums, k) do
    n = Enum.max(nums)
    n * k + div(k * (k - 1), 2)
  end
end
