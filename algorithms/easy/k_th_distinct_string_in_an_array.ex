defmodule Solution do
  @spec kth_distinct(arr :: [String.t()], k :: integer) :: String.t()
  def kth_distinct(arr, k) do
    counter = Enum.reduce(arr, %{}, fn n, acc -> Map.update(acc, n, 1, &(&1 + 1)) end)
    arr |> Enum.filter(&(counter[&1] == 1)) |> Enum.at(k - 1, "")
  end
end
