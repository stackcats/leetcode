defmodule Solution do
  @spec find_k_distant_indices(nums :: [integer], key :: integer, k :: integer) :: [integer]
  def find_k_distant_indices(nums, key, k) do
    keys =
      nums
      |> Enum.with_index()
      |> Enum.flat_map(fn {n, i} -> (n == key && [i]) || [] end)

    IO.inspect(keys)

    nums
    |> Enum.with_index()
    |> Enum.flat_map(fn {n, i} ->
      (Enum.any?(keys, fn key -> abs(i - key) <= k end) && [i]) || []
    end)
  end
end
