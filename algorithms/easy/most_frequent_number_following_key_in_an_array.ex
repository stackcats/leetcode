defmodule Solution do
  @spec most_frequent(nums :: [integer], key :: integer) :: integer
  def most_frequent(nums, key) do
    aux(nums, key)
    |> Enum.max_by(fn {_k, v} -> v end)
    |> elem(0)
  end

  def aux(nums, key), do: aux(nums, key, %{})

  def aux([_x], _key, map), do: map

  def aux([a, b | xs], key, map) do
    if a == key do
      aux([b | xs], key, Map.update(map, b, 1, &(&1 + 1)))
    else
      aux([b | xs], key, map)
    end
  end
end
