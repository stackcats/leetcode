defmodule RLEIterator do
  use GenServer
  
  @spec init_(encoding :: [integer]) :: any
  def init_(encoding) do
    GenServer.start_link(__MODULE__, [], name: __MODULE__)
    GenServer.call(__MODULE__, {:init, encoding})
  end

  @spec next(n :: integer) :: integer
  def next(n) do
    GenServer.call(__MODULE__, {:next, n})
  end

  def init(encoding) do
    {:ok, encoding}
  end

  def handle_call({:init, encoding}, _from, _state) do
      {:reply, [], encoding}
  end    
  def handle_call({:next, n}, _from, state) do
    {res, new_state} = next_(n, state)
    {:reply, res, new_state}
  end

  defp next_(_n, []), do: {-1, []}
  
  defp next_(n, [ct, num | rest]) do
    if ct >= n do
      {num, [ct - n, num | rest]}
    else
      next_(n - ct, rest)
    end
  end
end

# Your functions will be called as such:
# RLEIterator.init_(encoding)
# param_1 = RLEIterator.next(n)

# RLEIterator.init_ will be called before every test case, in which you can do some necessary initializations.
