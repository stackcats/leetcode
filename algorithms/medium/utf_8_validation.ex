defmodule Solution do
  use Bitwise

  @spec valid_utf8(data :: [integer]) :: boolean
  def valid_utf8([]), do: true

  def valid_utf8([x | xs]) do
    nbytes =
      cond do
        band(x, 0b10000000) == 0 -> 1
        band(x, 0b11100000) == 0b11000000 -> 2
        band(x, 0b11110000) == 0b11100000 -> 3
        band(x, 0b11111000) == 0b11110000 -> 4
        true -> 5
      end

    case nbytes do
      1 ->
        valid_utf8(xs)

      5 ->
        false

      n ->
        if check_rest(xs, n - 1) do
          xs |> Enum.drop(n - 1) |> valid_utf8()
        else
          false
        end
    end
  end

  def check_rest(xs, n) do
    ys = Enum.take(xs, n)
    length(ys) == n && Enum.all?(ys, fn y -> band(y, 0b11000000) == 0b10000000 end)
  end
end
