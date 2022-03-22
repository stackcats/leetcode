defmodule Solution do
  use Bitwise
  @spec xor_queries(arr :: [integer], queries :: [[integer]]) :: [integer]
  def xor_queries(arr, queries) do
    prefix =
      arr
      |> Enum.with_index()
      |> Enum.reduce(%{0 => 0}, fn {n, i}, acc ->
        Map.put(acc, i + 1, bxor(acc[i], n))
      end)

    queries
    |> Enum.map(fn [l, r] ->
      bxor(prefix[l], prefix[r + 1])
    end)
  end
end
