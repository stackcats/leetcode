defmodule Solution do
  @spec divide_array(nums :: [integer], k :: integer) :: [[integer]]
  def divide_array(nums, k) do
    nums
    |> Enum.sort()
    |> Stream.chunk_every(3)
    |> Enum.reduce_while([], fn [a, b, c] = lst, acc ->
      if c - a <= k do
        {:cont, [lst | acc]}
      else
        {:halt, []}
      end
    end)
  end
end
