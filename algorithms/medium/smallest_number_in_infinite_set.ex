defmodule SmallestInfiniteSet do
  use GenServer

  @spec init_() :: any
  def init_() do
    GenServer.start_link(__MODULE__, [], name: __MODULE__)
    GenServer.call(__MODULE__, :init)
  end

  @spec pop_smallest() :: integer
  def pop_smallest() do
    GenServer.call(__MODULE__, :pop)
  end

  @spec add_back(num :: integer) :: any
  def add_back(num) do
    GenServer.call(__MODULE__, {:add, num})
  end

  def init(state) do
    {:ok, state}
  end

  def handle_call(:init, _from, _state) do
    {:reply, true, {:gb_sets.empty(), 1}}
  end

  def handle_call(:pop, _from, {set, curr}) do
    if :gb_sets.is_empty(set) do
      {:reply, curr, {set, curr + 1}}
    else
      {r, set} = :gb_sets.take_smallest(set)
      {:reply, r, {set, curr}}
    end
  end

  def handle_call({:add, num}, _from, {set, curr}) do
    if num < curr && !:gb_sets.is_member(num, set) do
      {:reply, num, {:gb_sets.add(num, set), curr}}
    else
      {:reply, num, {set, curr}}
    end
  end
end

# Your functions will be called as such:
# SmallestInfiniteSet.init_()
# param_1 = SmallestInfiniteSet.pop_smallest()
# SmallestInfiniteSet.add_back(num)

# SmallestInfiniteSet.init_ will be called before every test case, in which you can do some necessary initializations.
