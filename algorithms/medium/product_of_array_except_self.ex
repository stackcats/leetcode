defmodule Solution do
  @spec product_except_self(nums :: [integer]) :: [integer]
  def product_except_self(nums) do
    prefix = nums |> product_prefix(1)
    suffix = nums |> Enum.reverse() |> product_prefix(1) |> Enum.reverse()
    Enum.zip(prefix, suffix) |> Enum.map(fn {a, b} -> a * b end)
  end

  def product_prefix([], _prefix), do: []

  def product_prefix([x | xs], prefix) do
    [prefix | product_prefix(xs, prefix * x)]
  end
end
