defmodule Solution do
  use Bitwise

  @spec find_array(pref :: [integer]) :: [integer]
  def find_array(pref), do: find_array(pref, 0, [])

  def find_array([], _acc, arr), do: arr |> Enum.reverse()

  def find_array([x | xs], acc, arr) do
    find_array(xs, x, [bxor(acc, x) | arr])
  end
end
