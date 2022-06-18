defmodule Solution do
  @spec calculate_tax(brackets :: [[integer]], income :: integer) :: float
  def calculate_tax(brackets, income) do
    brackets
    |> Enum.reduce_while({0, income, 0}, fn [u, p], {pre, income, ans} ->
      tmp = min(income, u - pre)
      ans = ans + tmp * p / 100
      income = income - tmp

      if income == 0 do
        {:halt, ans}
      else
        {:cont, {u, income, ans}}
      end
    end)
  end
end
