defmodule Solution do
  @spec num_of_pairs(nums :: [String.t], target :: String.t) :: integer
  def num_of_pairs(nums, target) do
    pairs = Enum.with_index(nums)
    for {e1, i1} <- pairs, {e2, i2} <- pairs, e1 <> e2 == target and i1 != i2, reduce: 0 do 
        acc -> acc + 1
    end
  end
end
