defmodule Solution do
  @spec count_operations(num1 :: integer, num2 :: integer) :: integer
  def count_operations(0, _num2), do: 0
  def count_operations(_num1, 0), do: 0

  def count_operations(num1, num2) do
    if num1 > num2 do
      1 + count_operations(num1 - num2, num2)
    else
      1 + count_operations(num1, num2 - num1)
    end
  end
end
