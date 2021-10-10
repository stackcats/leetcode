defmodule Solution do
  @spec find_duplicate(paths :: [String.t()]) :: [[String.t()]]
  def find_duplicate(paths) do
    paths
    |> Enum.reduce(%{}, &aux/2)
    |> Map.values()
    |> Enum.filter(&(Enum.count(&1) > 1))
  end

  def aux(path, m) do
    [dirname | files] = String.split(path)

    files
    |> Enum.map(&String.split(&1, ["(", ")"]))
    |> Enum.reduce(m, fn [filename, content | _rest], acc ->
      name = dirname <> "/" <> filename
      Map.update(acc, content, [name], &[name | &1])
    end)
  end
end
