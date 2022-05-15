defmodule Solution do
  @spec divisor_substrings(num :: integer, k :: integer) :: integer
  def divisor_substrings(num, k) do
    s = "#{num}"

    for i <- 0..(String.length(s) - k), reduce: 0 do
      acc ->
        n = s |> String.slice(i, k) |> String.to_integer()

        if n != 0 && rem(num, n) == 0 do
          acc + 1
        else
          acc
        end
    end
  end
end
