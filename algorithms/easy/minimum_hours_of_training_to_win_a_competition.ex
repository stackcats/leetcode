defmodule Solution do
  @spec min_number_of_hours(
          initial_energy :: integer,
          initial_experience :: integer,
          energy :: [integer],
          experience :: [integer]
        ) :: integer
  def min_number_of_hours(initial_energy, initial_experience, energy, experience) do
    ex_training =
      experience
      |> Enum.reduce({initial_experience, 0}, fn ex, {init, acc} ->
        if init <= ex do
          {init + ex + ex - init + 1, acc + ex - init + 1}
        else
          {init + ex, acc}
        end
      end)
      |> elem(1)

    all_energy = Enum.sum(energy)

    en_training = if initial_energy > all_energy, do: 0, else: all_energy - initial_energy + 1
    ex_training + en_training
  end
end
