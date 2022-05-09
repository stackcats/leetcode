defmodule Solution do
  @spec largest_good_integer(num :: String.t()) :: String.t()
  def largest_good_integer(num) when is_bitstring(num) do
    num |> String.graphemes() |> largest_good_integer("")
  end

  def largest_good_integer([], ans), do: ans
  def largest_good_integer([a], ans), do: ans
  def largest_good_integer([a, b], ans), do: ans

  def largest_good_integer([a, b, c | rest], ans) do
    if a == b && b == c do
      largest_good_integer(rest, max(ans, "#{a}#{b}#{c}"))
    else
      largest_good_integer([b, c | rest], ans)
    end
  end
end
