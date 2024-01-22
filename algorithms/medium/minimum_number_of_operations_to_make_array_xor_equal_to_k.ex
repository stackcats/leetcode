defmodule Solution do
  use Bitwise

  @spec min_operations(nums :: [integer], k :: integer) :: integer
  def min_operations(nums, k) do
    nums
    |> Enum.reduce(k, &bxor/2)
    |> Integer.digits(2)
    |> Enum.count(fn d -> d == 1 end)
  end
end
