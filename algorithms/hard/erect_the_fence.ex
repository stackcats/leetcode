defmodule Solution do
  @spec outer_trees(trees :: [[integer]]) :: [[integer]]
  def outer_trees(trees) do
    trees
    |> Enum.sort()
    |> Enum.reduce({[], []}, fn p, {lower, upper} ->
      {pick(p, lower, &>/2), pick(p, upper, &</2)}
    end)
    |> then(fn {l, u} ->
      l
      |> MapSet.new()
      |> MapSet.union(MapSet.new(u))
      |> MapSet.to_list()
    end)
  end

  def pick(p, [], _f), do: [p]
  def pick(p, [q], _f), do: [p, q]

  def pick(p, [q1, q2 | qs] = st, f) do
    if f.(orientation(q2, q1, p), 0) do
      pick(p, [q2 | qs], f)
    else
      [p | st]
    end
  end

  def orientation([x1, y1], [x2, y2], [x3, y3]) do
    (y3 - y2) * (x2 - x1) - (y2 - y1) * (x3 - x2)
  end
end
