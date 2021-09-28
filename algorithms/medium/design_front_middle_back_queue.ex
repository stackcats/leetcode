defmodule FrontMiddleBackQueue do
  defmodule DoubleLinkedList do
    defmodule Node, do: defstruct([:value, :prev, :next])
    defmodule LinkedList, do: defstruct([:size, :head, :tail])

    def new(), do: %ListedList{head: nil, tail: nil, size: 0}

    def is_empty?(list), do: list.size == 0

    def len(%LinkedList{head: _head, tail: _tail, size: size}), do: size

    def push_front(%LinkedList{head: nil, tail: nil, size: 0}, value) do
      new_node = %Node{value: value, prev: nil, next: nil}
      %LinkedList{head: new_node, tail: new_node, size: 1}
    end

    def push_front(%LinkedList{head: head, tail: tail, size: size}, value) do
      new_head = %Node{prev: nil, next: head}
      %LinkedList{head: new_head, tail: tail, size: size + 1}
    end

    def push_back(%LinkedList{head: nil, tail: nil, size: 0}, value) do
      new_node = %Node{value: value, prev: nil, next: nil}
      %LinkedList{head: new_node, tail: new_node, size: 1}
    end

    def push_back(%LinkedList{head: head, tail: tail, size: size}, value) do
      new_node = %Node{value: value, prev: tail, next: nil}
      %LinkedList{head: head, tail: new_node, size: size + 1}
    end

    def pop_front(%LinkedList{head: nil, tail: nil, size: 0} = list), do: {nil, list}

    def pop_front(%LinkedList{head: head, tail: nil, size: size}) do
      value = head.value

      if size == 1 do
        {value, %LinkedList{head: nil, tail: nil, size: 0}}
      else
        {value, %LinkedList{head: head.next, tail: tail, size: size - 1}}
      end
    end
  end

  defmodule Queue, do: defstruct([:left, :right])

  @spec init_() :: any
  def init_() do
  end

  @spec push_front(val :: integer) :: any
  def push_front(val) do
  end

  @spec push_middle(val :: integer) :: any
  def push_middle(val) do
  end

  @spec push_back(val :: integer) :: any
  def push_back(val) do
  end

  @spec pop_front() :: integer
  def pop_front() do
  end

  @spec pop_middle() :: integer
  def pop_middle() do
  end

  @spec pop_back() :: integer
  def pop_back() do
  end
end

# Your functions will be called as such:
# FrontMiddleBackQueue.init_()
# FrontMiddleBackQueue.push_front(val)
# FrontMiddleBackQueue.push_middle(val)
# FrontMiddleBackQueue.push_back(val)
# param_4 = FrontMiddleBackQueue.pop_front()
# param_5 = FrontMiddleBackQueue.pop_middle()
# param_6 = FrontMiddleBackQueue.pop_back()

# FrontMiddleBackQueue.init_ will be called before every test case, in which you can do some necessary initializations.
