defmodule Solution do
  @spec final_value_after_operations(operations :: [String.t]) :: integer
  def final_value_after_operations(operations) do
    List.foldl(operations, 0, fn(x, acc) -> acc + (if x == "++X" or x == "X++" do 1 else -1 end) end)
  end
end
