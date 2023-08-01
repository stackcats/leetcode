defmodule Solution do
  @spec number_of_employees_who_met_target(hours :: [integer], target :: integer) :: integer
  def number_of_employees_who_met_target(hours, target) do
    hours
    |> Enum.count(&(&1 >= target))
  end
end
