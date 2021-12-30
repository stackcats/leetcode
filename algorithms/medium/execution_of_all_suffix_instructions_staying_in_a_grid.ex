defmodule Solution do
  @spec execute_instructions(n :: integer, start_pos :: [integer], s :: String.t()) :: [integer]
  def execute_instructions(n, start_pos, s) do
    s = String.graphemes(s)

    0..(Enum.count(s) - 1)
    |> Enum.reduce([], fn i, acc -> [Enum.drop(s, i) | acc] end)
    |> Enum.reverse()
    |> Enum.map(fn s -> aux(n, start_pos, s, 0) end)
  end

  def aux(_n, _pos, [], ans), do: ans

  def aux(n, pos, [s | rest], ans) do
    pos = next_pos(pos, s)

    if is_valid_pos?(pos, n), do: aux(n, pos, rest, ans + 1), else: ans
  end

  def next_pos([x, y], "R"), do: [x, y + 1]
  def next_pos([x, y], "L"), do: [x, y - 1]
  def next_pos([x, y], "U"), do: [x - 1, y]
  def next_pos([x, y], "D"), do: [x + 1, y]

  def is_valid_pos?([x, y], n), do: x >= 0 && x < n && y >= 0 && y < n
end
