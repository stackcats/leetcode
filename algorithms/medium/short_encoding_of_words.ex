defmodule Solution do
  defmodule Trie do
    defstruct children: %{}

    def new(), do: %Trie{}

    def insert(nil, cs), do: insert(Trie.new(), cs)

    def insert(tree, []), do: tree

    def insert(tree, [c | cs]) do
      %{tree | children: Map.put(tree.children, c, insert(tree.children[c], cs))}
    end

    def dfs(tree, len, sum) do
      if map_size(tree.children) == 0 do
        sum + len + 1
      else
        tree.children
        |> Map.values()
        |> Enum.reduce(sum, fn v, acc ->
          dfs(v, len + 1, acc)
        end)
      end
    end
  end

  @spec minimum_length_encoding(words :: [String.t()]) :: integer
  def minimum_length_encoding(words) do
    words
    |> Enum.reduce(Trie.new(), fn w, acc ->
      w |> String.graphemes() |> Enum.reverse() |> then(&Trie.insert(acc, &1))
    end)
    |> Trie.dfs(0, 0)
  end
end
