defmodule Solution do
  @spec max_satisfaction(satisfaction :: [integer]) :: integer
  def max_satisfaction(satisfaction) do
    satisfaction
    |> Enum.sort(&>/2)
    |> Enum.reduce({0, 0, 0}, fn n, {max, sum, acc} ->
      acc = acc + n
      sum = sum + acc
      {max(max, sum), sum, acc}
    end)
    |> elem(0)
  end
end
