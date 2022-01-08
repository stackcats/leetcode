defmodule Solution do
  @spec number_of_beams(bank :: [String.t()]) :: integer
  def number_of_beams(bank) do
    bank
    |> Enum.reduce({0, 0}, fn row, {pre, ans} ->
      devices = row |> String.graphemes() |> Enum.count(&(&1 == "1"))

      if devices == 0 do
        {pre, ans}
      else
        {devices, ans + devices * pre}
      end
    end)
    |> elem(1)
  end
end
