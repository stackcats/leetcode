defmodule Solution do
  @spec change(amount :: integer, coins :: [integer]) :: integer
  def change(amount, coins) do
    for coin <- coins, reduce: %{0 => 1} do
      acc -> for i <- coin..amount, reduce: acc do
        acc -> Map.update(acc, i, acc[i-coin] || 0, &(&1 + (acc[i-coin] || 0)))
      end                             
    end |> Map.get(amount, 0)
  end      
end
