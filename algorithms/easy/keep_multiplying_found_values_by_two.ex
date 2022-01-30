defmodule Solution do
  @spec find_final_value(nums :: [integer], original :: integer) :: integer
  def find_final_value(nums, original) do
    set = MapSet.new(nums)

    Stream.iterate(original, &(&1 * 2))
    |> Stream.drop_while(&MapSet.member?(set, &1))
    |> Enum.take(1)
    |> hd()
  end
end
