defmodule Solution do
  @spec max_score_words(words :: [String.t()], letters :: [char], score :: [integer]) :: integer
  def max_score_words(words, letters, score) do
    words = Enum.map(words, &String.to_charlist/1)
    dfs(words, to_map(letters), score_map(score), 0, 0)
  end

  def dfs([], _letters, _score, curr, ans), do: max(curr, ans)

  def dfs([w | ws], letters, score, curr, ans) do
    if can_build_word(w, letters) do
      s = calc_score(w, score)
      letters = update_letters(w, letters, &(&1 - 1))
      ans = max(dfs(ws, letters, score, curr + s, ans), ans)
      letters = update_letters(w, letters, &(&1 + 1))
      dfs(ws, letters, score, curr, ans)
    else
      dfs(ws, letters, score, curr, ans)
    end
  end

  def update_letters(w, letters, f) do
    Enum.reduce(w, letters, fn c, m ->
      Map.update!(m, c, f)
    end)
  end

  def calc_score(w, score) do
    Enum.reduce(w, 0, fn c, acc ->
      acc + score[c]
    end)
  end

  def can_build_word(w, letters) do
    w
    |> to_map()
    |> Enum.all?(fn {k, v} ->
      Map.get(letters, k, 0) >= v
    end)
  end

  def to_map(lst) do
    lst
    |> Enum.reduce(%{}, fn c, m ->
      Map.update(m, c, 1, &(&1 + 1))
    end)
  end

  def score_map(score) do
    Enum.zip(?a..?z, score)
    |> Enum.reduce(%{}, fn {c, v}, m ->
      Map.put(m, c, v)
    end)
  end
end
