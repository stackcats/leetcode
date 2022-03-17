defmodule Cashier do
  use GenServer

  @spec init_(n :: integer, discount :: integer, products :: [integer], prices :: [integer]) ::
          any
  def init_(n, discount, products, prices) do
    GenServer.start_link(__MODULE__, [], name: __MODULE__)

    prices =
      Enum.zip(products, prices)
      |> Enum.reduce(%{}, fn {i, p}, acc ->
        Map.put(acc, i, p)
      end)

    GenServer.call(__MODULE__, {:init, {n, 0, discount, prices}})
  end

  @spec get_bill(product :: [integer], amount :: [integer]) :: float
  def get_bill(product, amount) do
    GenServer.call(__MODULE__, {:get_bill, product, amount})
  end

  def init(state) do
    {:ok, state}
  end

  def handle_call({:init, state}, _from, _state) do
    {:reply, true, state}
  end

  def handle_call({:get_bill, products, amounts}, _from, {n, ct, discount, prices}) do
    bill =
      Enum.zip(products, amounts)
      |> Enum.reduce(0, fn {i, amt}, acc -> acc + prices[i] * amt end)

    ct = ct + 1

    if rem(ct, n) == 0 do
      {:reply, bill * (1 - discount / 100), {n, ct, discount, prices}}
    else
      {:reply, bill, {n, ct, discount, prices}}
    end
  end
end
