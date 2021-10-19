# Definition for singly-linked list.
#
# defmodule ListNode do
#   @type t :: %__MODULE__{
#           val: integer,
#           next: ListNode.t() | nil
#         }
#   defstruct val: 0, next: nil
# end

defmodule Solution do
  use GenServer
  @spec init_(head :: ListNode.t() | nil) :: any
  def init_(head) do
    GenServer.start_link(__MODULE__, head, name: __MODULE__)
    GenServer.call(__MODULE__, {:init, head})
  end

  @spec get_random() :: integer
  def get_random() do
    GenServer.call(__MODULE__, :get_random)
  end

  def init(lst) do
    {:ok, lst}
  end

  def handle_call({:init, head}, _from, _state) do
    {:reply, nil, head}
  end

  def handle_call(:get_random, _from, %ListNode{val: val, next: next} = lst) do
    val = rand(lst, 1, val)
    {:reply, val, lst}
  end

  defp rand(nil, _n, res), do: res

  defp rand(%ListNode{val: val, next: next}, n, res) do
    if :rand.uniform(n) == n do
      rand(next, n + 1, val)
    else
      rand(next, n + 1, res)
    end
  end
end
