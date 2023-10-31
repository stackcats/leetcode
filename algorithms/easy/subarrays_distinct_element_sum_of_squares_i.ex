defmodule Solution do
  @spec sum_counts(nums :: [integer]) :: integer
  def sum_counts(nums) do
    len = length(nums)

    for chunk_size <- 1..len, reduce: 0 do
      acc ->
        Enum.chunk_every(nums, chunk_size, 1, :discard)
        |> Enum.reduce(0, fn chunk, acc ->
          chunk
          |> MapSet.new()
          |> MapSet.size()
          |> then(&(&1 * &1 + acc))
        end)
        |> then(&(&1 + acc))
    end
  end
end
