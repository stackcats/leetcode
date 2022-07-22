# TLE
defmodule TextEditor do
  use GenServer

  @spec init_() :: any
  def init_() do
    GenServer.start_link(__MODULE__, [], name: __MODULE__)
    GenServer.call(__MODULE__, :init)
  end

  @spec add_text(text :: String.t()) :: any
  def add_text(text) do
    GenServer.call(__MODULE__, {:add, text})
  end

  @spec delete_text(k :: integer) :: integer
  def delete_text(k) do
    GenServer.call(__MODULE__, {:delete, k})
  end

  @spec cursor_left(k :: integer) :: String.t()
  def cursor_left(k) do
    GenServer.call(__MODULE__, {:left, k})
  end

  @spec cursor_right(k :: integer) :: String.t()
  def cursor_right(k) do
    GenServer.call(__MODULE__, {:right, k})
  end

  def init(state) do
    {:ok, state}
  end

  def handle_call(:init, _from, _state) do
    {:reply, true, {[], []}}
  end

  def handle_call({:add, text}, _from, {p, s}) do
    p = text |> String.to_charlist() |> add(p)
    {:reply, true, {p, s}}
  end

  def handle_call({:delete, k}, _from, {p, s}) do
    ct = min(k, Enum.count(p))
    {:reply, ct, {Enum.drop(p, k), s}}
  end

  def handle_call({:left, k}, _from, {p, s}) do
    {p, s} = move(p, s, k)
    {:reply, left(p), {p, s}}
  end

  def handle_call({:right, k}, _from, {p, s}) do
    {s, p} = move(s, p, k)
    {:reply, left(p), {p, s}}
  end

  def add([], to), do: to

  def add([f | from], to) do
    add(from, [f | to])
  end

  def move([], to, _k), do: {[], to}

  def move(from, to, 0), do: {from, to}

  def move([f | from], to, k) do
    move(from, [f | to], k - 1)
  end

  def left(from), do: left(from, 10, [])
  def left([], _k, to), do: List.to_string(to)
  def left(_from, 0, to), do: List.to_string(to)

  def left([f | from], k, to) do
    left(from, k - 1, [f | to])
  end
end
