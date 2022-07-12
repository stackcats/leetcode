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
  @spec spiral_matrix(m :: integer, n :: integer, head :: ListNode.t() | nil) :: [[integer]]
  def spiral_matrix(m, n, head) do
    dt = aux(%{}, head, {0, 0, :right}, m, n)

    for i <- 0..(m - 1) do
      for j <- 0..(n - 1), do: dt[{i, j}] || -1
    end
  end

  def aux(acc, nil, _pos, _m, _n), do: acc

  def aux(acc, _head, _pos, m, n) when map_size(acc) == m * n do
    acc
  end

  def aux(acc, head, {x, y, dir}, m, n) do
    if x < 0 || x >= m || y < 0 || y >= n || acc[{x, y}] != nil do
      aux(acc, head, turn({x, y, dir}), m, n)
    else
      {val, head} = if head, do: {head.val, head.next}, else: {-1, nil}

      Map.put(acc, {x, y}, val) |> aux(head, next({x, y, dir}), m, n)
    end
  end

  def turn({x, y, dir}) do
    case dir do
      :up -> {x + 1, y + 1, :right}
      :down -> {x - 1, y - 1, :left}
      :left -> {x - 1, y + 1, :up}
      :right -> {x + 1, y - 1, :down}
    end
  end

  def next({x, y, dir}) do
    case dir do
      :up -> {x - 1, y, dir}
      :down -> {x + 1, y, dir}
      :left -> {x, y - 1, dir}
      :right -> {x, y + 1, dir}
    end
  end
end
