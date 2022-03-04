defmodule Solution do
  @spec count_even(num :: integer) :: integer
  def count_even(num) do
    for n <- 1..num, is_even(digit_sum(n)) do
      1
    end
    |> Enum.sum()
  end

  def digit_sum(0), do: 0

  def digit_sum(num) do
    rem(num, 10) + digit_sum(div(num, 10))
  end

  def is_even(num) do
    rem(num, 2) == 0
  end
end
