defmodule Solution do
  use Agent

  @spec partition(s :: String.t()) :: [[String.t()]]
  def partition(s) do
    dfs(s, String.length(s), 0, [], [])
  end

  def dfs(_s, len, l, subs, ans) when l == len do
    [Enum.reverse(subs) | ans]
  end

  def dfs(s, len, l, subs, ans) do
    l..(len - 1)
    |> Enum.reduce(ans, fn r, ans ->
      ss = String.slice(s, l..r)

      if ss == String.reverse(ss) do
        dfs(s, len, r + 1, [ss | subs], ans)
      else
        ans
      end
    end)
  end
end
