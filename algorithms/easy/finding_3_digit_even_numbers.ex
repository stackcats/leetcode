import Integer

defmodule Solution do
  @spec find_even_numbers(digits :: [integer]) :: [integer]
  def find_even_numbers(digits) do
    map =
      Enum.reduce(digits, %{}, fn d, acc ->
        Map.update(acc, d, 1, &(&1 + 1))
      end)

    100..998//2 |> Enum.filter(fn n -> is_3_digit_even(n, map) end) |> Enum.sort()
  end

  def is_3_digit_even(n, map) do
    integer_to_map(n, %{})
    |> Enum.all?(fn {k, v} ->
      Map.has_key?(map, k) && map[k] >= v
    end)
  end

  def integer_to_map(0, m), do: m

  def integer_to_map(n, m) do
    integer_to_map(div(n, 10), Map.update(m, rem(n, 10), 1, &(&1 + 1)))
  end
end
