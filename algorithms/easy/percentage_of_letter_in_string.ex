defmodule Solution do
  @spec percentage_letter(s :: String.t(), letter :: char) :: integer
  def percentage_letter(s, letter) do
    s
    |> String.to_charlist()
    |> Enum.reduce({0, 0}, fn c, {ct, all} ->
      if c == letter, do: {ct + 1, all + 1}, else: {ct, all + 1}
    end)
    |> then(fn {ct, all} ->
      (ct * 100 / all) |> Float.floor(2) |> trunc()
    end)
  end
end
