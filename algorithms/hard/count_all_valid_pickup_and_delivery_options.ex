defmodule Solution do
  @spec count_orders(n :: integer) :: integer
  def count_orders(n) do
    p =
      for i <- 1..n, reduce: 1 do
        acc -> (acc * i) |> rem(1_000_000_007)
      end

    for i <- 1..(2 * n)//2, reduce: p do
      acc -> (acc * (2 * n - i)) |> rem(1_000_000_007)
    end
  end
end
