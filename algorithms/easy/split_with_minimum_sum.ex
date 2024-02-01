defmodule Solution do
  @spec split_num(num :: integer) :: integer
  def split_num(num) do
    num
    |> Integer.digits()
    |> Enum.sort()
    |> Enum.chunk_every(2)
    |> Enum.reduce([0, 0], fn each, [xs, ys] ->
      case each do
        [x, y] -> [xs * 10 + x, ys * 10 + y]
        [x] -> [xs * 10 + x, ys]
      end
    end)
    |> Enum.sum()
  end
end
