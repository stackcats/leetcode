defmodule Solution do
  @spec max_profit(prices :: [integer]) :: integer
  def max_profit([p | _] = prices) do
    aux(prices, -p, 0, -p, 0)
  end

  def aux([], _buy1, _sell1, _buy2, sell2), do: sell2

  def aux([price | rest], buy1, sell1, buy2, sell2) do
    aux(
      rest,
      max(buy1, -price),
      max(sell1, buy1 + price),
      max(buy2, sell1 - price),
      max(sell2, buy2 + price)
    )
  end
end
