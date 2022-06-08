defmodule Solution do
  @spec min_max_game(nums :: [integer]) :: integer
  def min_max_game([n]), do: n

  def min_max_game(nums) do
    nums
    |> Enum.chunk_every(2)
    |> Enum.flat_map_reduce(:min, fn [a, b], acc ->
      case acc do
        :min -> {[min(a, b)], :max}
        _ -> {[max(a, b)], :min}
      end
    end)
    |> elem(0)
    |> min_max_game()
  end
end
