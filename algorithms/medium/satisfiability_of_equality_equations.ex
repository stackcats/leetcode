defmodule Solution do
  defmodule DSet do
    defstruct roots: %{}, ranks: %{}

    def find(set, x) do
      if x == Map.get(set.roots, x, x), do: x, else: find(set, set.roots[x])
    end

    def union(set, x, y) do
      rx = find(set, x)
      ry = find(set, y)

      cond do
        rx == ry ->
          set

        set.ranks[rx] > set.ranks[ry] ->
          %{set | roots: Map.put(set.roots, ry, rx)}

        set.ranks[rx] < set.ranks[ry] ->
          %{set | roots: Map.put(set.roots, rx, ry)}

        true ->
          %DSet{
            roots: Map.put(set.roots, rx, ry),
            ranks: Map.update(set.ranks, rx, 1, &(&1 + 1))
          }
      end
    end
  end

  @spec equations_possible(equations :: [String.t()]) :: boolean
  def equations_possible(equations) do
    {eqs, nes} = equations |> Enum.split_with(fn eq -> Regex.match?(~r/==/, eq) end)

    eqs
    |> Enum.reduce(%DSet{}, fn eq, set ->
      [a, _, _, b] = eq |> String.graphemes()
      DSet.union(set, a, b)
    end)
    |> then(fn set ->
      nes
      |> Enum.all?(fn eq ->
        [a, _, _, b] = eq |> String.graphemes()
        DSet.find(set, a) != DSet.find(set, b)
      end)
    end)
  end
end
