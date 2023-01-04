defmodule Solution do
  @spec capture_forts(forts :: [integer]) :: integer
  def capture_forts(forts) do
    forts
    |> Enum.with_index()
    |> Enum.reduce({0, 0, 0}, fn {n, i}, {j, prev, ans} ->
      cond do
        n == 0 -> {j, prev, ans}
        n == -prev -> {i, n, max(ans, i - j - 1)}
        true -> {i, n, ans}
      end
    end)
    |> elem(2)
  end
end
