defmodule Solution do
  @spec valid_tic_tac_toe(board :: [String.t()]) :: boolean
  def valid_tic_tac_toe(board) do
    b =
      board
      |> Enum.join()
      |> String.graphemes()
      |> Enum.with_index()
      |> Enum.reduce(%{}, fn {c, i}, acc ->
        Map.put(acc, i, c)
      end)

    {x, o} =
      b
      |> Enum.reduce({0, 0}, fn {_, c}, {x, o} ->
        case c do
          "X" -> {x + 1, o}
          "O" -> {x, o + 1}
          _ -> {x, o}
        end
      end)

    cond do
      x == o -> not num_of_win(b, "X")
      x == o + 1 -> not num_of_win(b, "O")
      true -> false
    end
  end

  def num_of_win(b, c) do
    [[0, 1, 2], [3, 4, 5], [6, 7, 8], [0, 3, 6], [1, 4, 7], [2, 5, 8], [0, 4, 8], [2, 4, 6]]
    |> Enum.any?(fn each -> is_win?(b, each, c) end)
  end

  def is_win?(b, is, c) do
    Enum.all?(is, fn i -> b[i] == c end)
  end
end
