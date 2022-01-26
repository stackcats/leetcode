defmodule Solution do
  use Bitwise

  @spec get_maximum_xor(nums :: [integer], maximum_bit :: integer) :: [integer]
  def get_maximum_xor(nums, maximum_bit) do
    xor = Enum.reduce(nums, 0, fn n, acc -> bxor(acc, n) end)
    mask = (1 <<< maximum_bit) - 1

    nums
    |> Enum.reverse()
    |> Enum.reduce({[], xor}, fn n, {acc, xor} ->
      {[bxor(xor, mask) | acc], bxor(xor, n)}
    end)
    |> elem(0)
    |> Enum.reverse()
  end
end
