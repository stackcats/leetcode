defmodule Solution do
  @spec count_digits(num :: integer) :: integer
  def count_digits(num), do: count_digits(num, num, 0)
  def count_digits(0, _, ans), do: ans

  def count_digits(n, num, ct) do
    d = div(n, 10)
    r = rem(n, 10)

    if rem(num, r) == 0 do
      count_digits(d, num, ct + 1)
    else
      count_digits(d, num, ct)
    end
  end
end
