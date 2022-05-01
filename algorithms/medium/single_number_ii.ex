defmodule Solution do
  use Bitwise
  @spec single_number(nums :: [integer]) :: integer
  def single_number(nums) do
    nums
    |> Enum.reduce({0, 0}, fn n, {ones, twos} ->
      ones = bxor(ones, n) &&& bnot(twos)
      twos = bxor(twos, n) &&& bnot(ones)
      {ones, twos}
    end)
    |> elem(0)
  end
end
