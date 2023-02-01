defmodule Solution do
  @spec sort_the_students(score :: [[integer]], k :: integer) :: [[integer]]
  def sort_the_students(score, k) do
    score |> Enum.sort_by(&(-Enum.at(&1, k)))
  end
end
