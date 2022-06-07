defmodule Solution do
  defmodule DSet do
    defstruct root: %{}, rank: %{}

    def new(n) do
      {root, rank} =
        0..(n - 1)
        |> Enum.reduce({%{}, %{}}, fn i, {root, rank} ->
          {Map.put(root, i, i), Map.put(rank, i, 1)}
        end)

      %DSet{root: root, rank: rank}
    end

    def find(ds, x) do
      if ds.root[x] == x do
        {x, ds}
      else
        {r, ds} = find(ds, ds.root[x])
        {r, %{ds | root: Map.put(ds.root, x, r)}}
      end
    end

    def union(ds, x, y) do
      {rootX, ds} = find(ds, x)
      {rootY, ds} = find(ds, y)

      cond do
        rootX == rootY ->
          ds

        ds.root[rootX] > ds.root[rootY] ->
          %{ds | root: Map.put(ds.root, rootY, rootX)}

        ds.root[rootX] < ds.root[rootY] ->
          %{ds | root: Map.put(ds.root, rootX, rootY)}

        true ->
          %DSet{
            root: Map.put(ds.root, rootX, rootY),
            rank: Map.update!(ds.rank, rootX, &(&1 + 1))
          }
      end
    end
  end

  @spec find_circle_num(is_connected :: [[integer]]) :: integer
  def find_circle_num(is_connected) do
    n = Enum.count(is_connected)

    ds =
      is_connected
      |> Enum.with_index()
      |> Enum.reduce(DSet.new(n), fn {row, i}, ds ->
        row
        |> Enum.with_index()
        |> Enum.reduce(ds, fn {x, j}, ds ->
          if x == 1, do: DSet.union(ds, i, j), else: ds
        end)
      end)

    0..(n - 1)
    |> Enum.reduce(MapSet.new(), fn i, acc ->
      DSet.find(ds, i) |> elem(0) |> then(&MapSet.put(acc, &1))
    end)
    |> MapSet.size()
  end
end
