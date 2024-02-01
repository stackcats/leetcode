defmodule Solution do
  @spec buy_choco(prices :: [integer], money :: integer) :: integer
  def buy_choco([a, b | ps], money) do
    ps
    |> Enum.reduce([a, b], fn p, acc ->
      [p | acc] |> Enum.sort() |> Enum.take(2)
    end)
    |> Enum.sum()
    |> then(fn sum ->
      if sum <= money, do: money - sum, else: money
    end)
  end
end
