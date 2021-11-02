defmodule Solution do
  @spec max_profit(prices :: [integer], fee :: integer) :: integer
  def max_profit([price | rest], fee) do
    aux(rest, -price - fee, 0, fee)
  end

  @spec aux(prices :: [integer], bought :: integer, sold :: integer, fee :: integer) :: integer
  def aux([], _bought, sold, _fee), do: sold

  def aux([price | rest], bought, sold, fee) do
    aux(rest, max(bought, sold - price - fee), max(sold, bought + price), fee)
  end
end
