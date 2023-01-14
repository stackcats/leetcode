defmodule Solution do
  @spec smallest_equivalent_string(s1 :: String.t(), s2 :: String.t(), base_str :: String.t()) ::
          String.t()
  def smallest_equivalent_string(s1, s2, base_str) do
    s1 = String.graphemes(s1)
    s2 = String.graphemes(s2)

    uf =
      [s1, s2]
      |> Enum.zip()
      |> Enum.reduce(%{}, fn {c1, c2}, acc ->
        union(acc, c1, c2)
      end)

    base_str
    |> String.graphemes()
    |> Enum.map(fn c -> find(uf, c) |> elem(0) end)
    |> Enum.join()
  end

  def union(uf, c1, c2) do
    {r1, uf} = find(uf, c1)
    {r2, uf} = find(uf, c2)

    cond do
      r1 == r2 -> uf
      r1 < r2 -> Map.put(uf, r2, r1)
      true -> Map.put(uf, r1, r2)
    end
  end

  def find(uf, c) do
    r = uf[c]

    case r do
      nil -> {c, Map.put(uf, c, c)}
      ^c -> {c, uf}
      _ -> find(uf, r)
    end
  end
end
