defmodule Solution do
  @spec find_cheapest_price(
          n :: integer,
          flights :: [[integer]],
          src :: integer,
          dst :: integer,
          k :: integer
        ) :: integer
  def find_cheapest_price(n, flights, src, dst, k) do
    graph =
      flights
      |> Enum.reduce(%{}, fn [from, to, price], acc ->
        x = {to, price}
        Map.update(acc, from, [x], &[x | &1])
      end)

    :queue.new()
    |> :queue.snoc({src, k, 0})
    |> bfs(graph)
    |> Map.get(dst, -1)
  end

  def bfs(q, graph, mp \\ %{}) do
    if :queue.is_empty(q) do
      mp
    else
      {curr, ct, price} = :queue.head(q)
      q = :queue.tail(q)

      cond do
        Map.get(mp, curr, price + 1) < price ->
          bfs(q, graph, mp)

        ct < 0 ->
          bfs(q, graph, Map.put(mp, curr, price))

        true ->
          Map.get(graph, curr, [])
          |> Enum.reduce(q, fn {dst, p}, q ->
            :queue.in({dst, ct - 1, p + price}, q)
          end)
          |> bfs(graph, Map.put(mp, curr, price))
      end
    end
  end
end
