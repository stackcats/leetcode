defmodule Solution do
  @spec sort_array(nums :: [integer]) :: [integer]
  def sort_array([]), do: []

  def sort_array(lst) do
    lst |> List.pop_at(rand_pos(lst)) |> quicksort()
  end

  defp quicksort({nil, _rest}), do: []

  defp quicksort({pivot, []}), do: [pivot]

  defp quicksort({pivot, rest}) do
    {left, right} = Enum.split_with(rest, &(&1 <= pivot))

    quicksort(List.pop_at(left, rand_pos(left))) ++
      [pivot] ++ quicksort(List.pop_at(right, rand_pos(right)))
  end

  defp rand_pos([]), do: 0

  defp rand_pos(lst), do: :rand.uniform(length(lst)) - 1
end
