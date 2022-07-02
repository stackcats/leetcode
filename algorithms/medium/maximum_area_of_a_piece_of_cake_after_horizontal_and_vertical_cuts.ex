defmodule Solution do
  @spec max_area(
          h :: integer,
          w :: integer,
          horizontal_cuts :: [integer],
          vertical_cuts :: [integer]
        ) :: integer
  def max_area(h, w, horizontal_cuts, vertical_cuts) do
    (max_diff([h | horizontal_cuts]) * max_diff([w | vertical_cuts]))
    |> rem(1_000_000_000)
  end

  def max_diff(lst) do
    lst
    |> Enum.sort()
    |> Enum.reduce({0, 0}, fn h, {m, p} ->
      {max(m, h - p), h}
    end)
    |> elem(0)
  end
end
