defmodule Solution do
  @spec find_closest_elements(arr :: [integer], k :: integer, x :: integer) :: [integer]
  def find_closest_elements(arr, k, x) do
    arr
    |> Enum.reduce(:gb_sets.new(), fn n, acc ->
      acc = :gb_sets.add({abs(n - x), n, :rand.uniform()}, acc)

      if :gb_sets.size(acc) > k do
        {_, acc} = :gb_sets.take_largest(acc)
        acc
      else
        acc
      end
    end)
    |> :gb_sets.to_list()
    |> Enum.map(fn {_, n, _} -> n end)
    |> Enum.sort()
  end
end
