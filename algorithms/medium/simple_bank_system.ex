defmodule Bank do
  use GenServer

  @spec init_(balance :: [integer]) :: any
  def init_(balance) do
    GenServer.start_link(__MODULE__, [], name: __MODULE__)
    balance = balance |> Enum.with_index(fn e, i -> {i + 1, e} end) |> Map.new()
    GenServer.call(__MODULE__, {:init, balance})
  end

  @spec transfer(account1 :: integer, account2 :: integer, money :: integer) :: boolean
  def transfer(account1, account2, money) do
    GenServer.call(__MODULE__, {:transfer, account1, account2, money})
  end

  @spec deposit(account :: integer, money :: integer) :: boolean
  def deposit(account, money) do
    GenServer.call(__MODULE__, {:deposit, account, money})
  end

  @spec withdraw(account :: integer, money :: integer) :: boolean
  def withdraw(account, money) do
    GenServer.call(__MODULE__, {:withdraw, account, money})
  end

  # Server
  def init(state) do
    {:ok, state}
  end

  def handle_call({:init, balance}, _from, _state) do
    {:reply, true, balance}
  end

  def handle_call({:transfer, account1, account2, money}, _from, balance) do
    cond do
      not Map.has_key?(balance, account1) ->
        {:reply, false, balance}

      not Map.has_key?(balance, account2) ->
        {:reply, false, balance}

      balance[account1] < money ->
        {:reply, false, balance}

      true ->
        balance = Map.update!(balance, account1, &(&1 - money))
        balance = Map.update!(balance, account2, &(&1 + money))
        {:reply, true, balance}
    end
  end

  def handle_call({:deposit, account, money}, _from, balance) do
    cond do
      not Map.has_key?(balance, account) ->
        {:reply, false, balance}

      true ->
        balance = Map.update!(balance, account, &(&1 + money))
        {:reply, true, balance}
    end
  end

  def handle_call({:withdraw, account, money}, _from, balance) do
    cond do
      not Map.has_key?(balance, account) ->
        {:reply, false, balance}

      balance[account] < money ->
        {:reply, false, balance}

      true ->
        balance = Map.update!(balance, account, &(&1 - money))
        {:reply, true, balance}
    end
  end
end

# Your functions will be called as such:
# Bank.init_(balance)
# param_1 = Bank.transfer(account1, account2, money)
# param_2 = Bank.deposit(account, money)
# param_3 = Bank.withdraw(account, money)

# Bank.init_ will be called before every test case, in which you can do some necessary initializations.
