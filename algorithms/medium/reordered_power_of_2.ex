defmodule Solution do
  @spec reordered_power_of2(n :: integer) :: boolean
  def reordered_power_of2(n) do
    m = count(n)

    0..32
    |> Enum.any?(fn i ->
      count(Integer.pow(2, i)) == m
    end)
  end

  def count(n), do: count(n, %{})
  def count(0, ct), do: ct

  def count(n, ct) do
    count(div(n, 10), Map.update(ct, rem(n, 10), 1, &(&1 + 1)))
  end
end
