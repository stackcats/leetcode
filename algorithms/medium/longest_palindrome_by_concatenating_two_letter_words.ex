defmodule Solution do
  @spec longest_palindrome(words :: [String.t()]) :: integer
  def longest_palindrome(words) do
    words
    |> Enum.reduce({%{}, 0}, fn w, {mp, ct} ->
      rw = String.reverse(w)

      if Map.get(mp, rw, 0) > 0 do
        {Map.update!(mp, rw, &(&1 - 1)), ct + 4}
      else
        {Map.update(mp, w, 1, &(&1 + 1)), ct}
      end
    end)
    |> then(fn {mp, ct} ->
      if Enum.any?(mp, fn {k, v} -> v > 0 && String.reverse(k) == k end) do
        ct + 2
      else
        ct
      end
    end)
  end
end
