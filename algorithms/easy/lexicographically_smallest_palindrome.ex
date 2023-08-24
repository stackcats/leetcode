defmodule Solution do
  @spec make_smallest_palindrome(s :: String.t()) :: String.t()
  def make_smallest_palindrome(s) do
    s = String.graphemes(s)
    t = Enum.reverse(s)

    Enum.zip(s, t)
    |> Enum.map(fn {a, b} -> min(a, b) end)
    |> Enum.join()
  end
end
