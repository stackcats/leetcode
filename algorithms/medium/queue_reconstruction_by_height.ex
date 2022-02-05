defmodule Solution do
  @spec reconstruct_queue(people :: [[integer]]) :: [[integer]]
  def reconstruct_queue(people) do
    people
    |> Enum.sort(fn [hi, ki], [hj, kj] ->
      if hi == hj, do: ki < kj, else: hi > hj
    end)
    |> aux()
  end

  def aux(lst), do: aux(lst, [])
  def aux([], lst), do: lst

  def aux([[_, h] = x | xs], lst) do
    aux(xs, List.insert_at(lst, h, x))
  end
end
