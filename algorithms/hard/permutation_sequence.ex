defmodule Solution do
  @spec get_permutation(n :: integer, k :: integer) :: String.t()
  def get_permutation(n, k) do
    fs =
      for i <- 1..n, reduce: %{0 => 1} do
        m -> Map.put(m, i, m[i - 1] * i)
      end

    lst = Enum.into(1..n, [])

    for i <- 1..n, reduce: {[], lst, k - 1} do
      {ans, lst, k} ->
        ndx = div(k, fs[n - i])
        {x, lst} = List.pop_at(lst, ndx)
        {["#{x}" | ans], lst, k - ndx * fs[n - i]}
    end
    |> elem(0)
    |> Enum.reverse()
    |> Enum.join()
  end
end
