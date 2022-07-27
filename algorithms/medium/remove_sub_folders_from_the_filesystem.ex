defmodule Solution do
  @spec remove_subfolders(folder :: [String.t()]) :: [String.t()]
  def remove_subfolders(folder) do
    folder
    |> Enum.sort()
    |> Enum.reduce([], fn f, acc ->
      cond do
        acc == [] -> [f]
        String.starts_with?(f, hd(acc) <> "/") -> acc
        true -> [f | acc]
      end
    end)
  end
end
