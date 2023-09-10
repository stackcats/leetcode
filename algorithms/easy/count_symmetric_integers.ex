defmodule Solution do
  @spec count_symmetric_integers(low :: integer, high :: integer) :: integer
  def count_symmetric_integers(low, high) do
    low..high
    |> Enum.reduce(0, fn i, acc ->
      if is_symmetric(i) do
        acc + 1
      else
        acc
      end
    end)
  end

  def is_symmetric(n) do
    arr =
      "#{n}"
      |> String.graphemes()
      |> Enum.map(&String.to_integer/1)

    len = length(arr)

    if rem(len, 2) == 1 do
      false
    else
      {left, right} = arr |> Enum.split(div(len, 2))
      Enum.sum(left) == Enum.sum(right)
    end
  end
end
