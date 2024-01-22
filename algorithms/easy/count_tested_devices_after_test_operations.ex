defmodule Solution do
  @spec count_tested_devices(battery_percentages :: [integer]) :: integer
  def count_tested_devices(battery_percentages) do
    battery_percentages
    |> Enum.reduce(0, fn bp, acc ->
      if bp > acc, do: acc + 1, else: acc
    end)
  end
end
