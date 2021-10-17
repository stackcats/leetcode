defmodule Solution do
  @spec min_moves_to_seat(seats :: [integer], students :: [integer]) :: integer
  def min_moves_to_seat(seats, students) do
    seats = Enum.sort(seats)
    students = Enum.sort(students)
    Enum.zip_with([seats, students], fn [x, y] -> abs(x - y) end) |> Enum.sum()
  end
end
