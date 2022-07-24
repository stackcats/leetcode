defmodule Solution do
  defmodule FenwickTree do
    use Bitwise
    defstruct v: %{}, n: 0

    def new(n) do
      %FenwickTree{n: n}
    end

    def add(%FenwickTree{v: v, n: n} = t, i, val) when i > n, do: t

    def add(t, i, val) do
      nv = Map.update(t.v, i, val, &(&1 + val))
      add(%FenwickTree{t | v: nv}, i + lowbit(i), val)
    end

    def query(t, i) do
      query(t, i, 0)
    end

    def query(%FenwickTree{v: v, n: n}, i, sum) when i <= 0, do: sum

    def query(t, i, sum) do
      query(t, i - lowbit(i), sum + Map.get(t.v, i, 0))
    end

    defp lowbit(n) do
      n &&& -n
    end
  end

  @spec count_smaller(nums :: [integer]) :: [integer]
  def count_smaller(nums) do
    offset = 10000

    nums
    |> Enum.reverse()
    |> Enum.reduce({[], FenwickTree.new(2 * offset + 10)}, fn n, {ans, t} ->
      {[FenwickTree.query(t, n + offset) | ans], FenwickTree.add(t, n + offset + 1, 1)}
    end)
    |> elem(0)
  end
end
