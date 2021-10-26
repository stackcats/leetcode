defmodule Solution do
  @spec check_powers_of_three(n :: integer) :: boolean
  def check_powers_of_three(0), do: false
  def check_powers_of_three(1), do: true
  def check_powers_of_three(2), do: false
  def check_powers_of_three(3), do: true
  def check_powers_of_three(4), do: true

  def check_powers_of_three(n) do
    (rem(n, 3) == 0 && check_powers_of_three(div(n, 3))) ||
      (rem(n - 1, 3) == 0 && check_powers_of_three(div(n - 1, 3)))
  end
end
