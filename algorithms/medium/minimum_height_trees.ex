defmodule Solution do
  @spec find_min_height_trees(n :: integer, edges :: [[integer]]) :: [integer]
  def find_min_height_trees(1, _), do: [0]

  def find_min_height_trees(n, edges) do
    {deg, graph} = generate_graph(edges, %{}, %{})
    leaves = deg |> Enum.filter(fn {_, v} -> v == 1 end) |> Enum.map(fn {k, v} -> k end)
    last_leaves(n, leaves, deg, graph)
  end

  def last_leaves(n, leaves, _, _) when n <= 2, do: leaves

  def last_leaves(n, leaves, deg, graph) do
    {new_deg, new_leaves} = eliminate_leaves(leaves, graph, deg, [])
    last_leaves(n - Enum.count(leaves), new_leaves, new_deg, graph)
  end

  def eliminate_leaves([], graph, deg, leaves), do: {deg, leaves}

  def eliminate_leaves([leaf | rest], graph, deg, leaves) do
    {new_deg, new_leaves} = update_degrees(graph[leaf], deg, leaves)
    eliminate_leaves(rest, graph, new_deg, new_leaves)
  end

  def update_degrees([], deg, leaves), do: {deg, leaves}

  def update_degrees([n | rest], deg, leaves) do
    deg = Map.update(deg, n, 0, fn d -> d - 1 end)

    if deg[n] == 1 do
      update_degrees(rest, deg, [n | leaves])
    else
      update_degrees(rest, deg, leaves)
    end
  end

  def generate_graph([], degrees, graph) do
    {degrees, graph}
  end

  def generate_graph([[n1, n2] | rest], degrees, graph) do
    degrees =
      degrees
      |> Map.update(n1, 1, fn n -> n + 1 end)
      |> Map.update(n2, 1, fn n -> n + 1 end)

    graph =
      graph
      |> Map.update(n1, [n2], fn lst -> [n2 | lst] end)
      |> Map.update(n2, [n1], fn lst -> [n1 | lst] end)

    generate_graph(rest, degrees, graph)
  end
end
