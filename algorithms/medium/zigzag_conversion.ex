defmodule Solution do
  @spec convert(s :: String.t(), num_rows :: integer) :: String.t()
  def convert(s, num_rows) do
    len = String.length(s)

    if len == 1 || len <= num_rows do
      s
    else
      s
      |> String.graphemes()
      |> Enum.reduce({%{}, 0, 1}, fn c, {mp, i, step} ->
        step =
          cond do
            i == 0 -> 1
            i == num_rows - 1 -> -1
            true -> step
          end

        {Map.update(mp, i, [c], &[c | &1]), i + step, step}
      end)
      |> elem(0)
      |> Enum.sort_by(fn {k, v} -> k end)
      |> Enum.map(fn {k, v} -> Enum.reverse(v) end)
      |> Enum.join()
    end
  end
end
