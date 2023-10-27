defmodule Solution do
  @spec difference_of_sums(n :: integer, m :: integer) :: integer
  def difference_of_sums(n, m) do
    1..n
    |> Enum.reduce(0, fn i, acc ->
      acc + if rem(i, m) == 0, do: -i, else: i
    end)
  end
end
