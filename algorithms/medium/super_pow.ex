defmodule Solution do
  @spec super_pow(a :: integer, b :: [integer]) :: integer
  def super_pow(a, b) do
    Enum.reduce(b, 1, fn n, acc -> (pow_mod_1337(acc, 10) * pow_mod_1337(a, n)) |> rem(1337) end)
  end

  def pow_mod_1337(a, b), do: Integer.pow(a, b) |> rem(1337)
end
