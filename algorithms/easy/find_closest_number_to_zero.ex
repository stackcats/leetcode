defmodule Solution do
  @spec find_closest_number(nums :: [integer]) :: integer
  def find_closest_number([x | xs]) do
    xs
    |> Enum.reduce(x, fn n, acc ->
      cond do
        abs(n) < abs(acc) -> n
        abs(n) > abs(acc) -> acc
        true -> max(n, acc)
      end
    end)
  end
end
