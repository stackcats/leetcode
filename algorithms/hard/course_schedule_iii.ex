defmodule Solution do
  defmodule Queue do
    defstruct data: nil, size: 0

    @leaf nil

    def new(), do: %Queue{}

    defp rank(@leaf), do: 0
    defp rank({r, _, _, _}), do: r

    defp merge(@leaf, t), do: t
    defp merge(t, @leaf), do: t

    defp merge({_, lv, ll, lr} = t1, {_, rv, rl, rr} = t2) do
      if lv >= rv do
        swipe(lv, ll, merge(lr, t2))
      else
        swipe(rv, rl, merge(t1, rr))
      end
    end

    defp swipe(v, left, right) do
      if rank(left) >= rank(right) do
        {rank(right) + 1, v, left, right}
      else
        {rank(left) + 1, v, right, left}
      end
    end

    def push(%Queue{data: data, size: size}, v) do
      %Queue{data: merge(data, {1, v, @leaf, @leaf}), size: size + 1}
    end

    def pop(%Queue{data: {_, v, l, r}, size: size}) do
      {v, %Queue{data: merge(l, r), size: size - 1}}
    end

    def size(q), do: q.size
  end

  @spec schedule_course(courses :: [[integer]]) :: integer
  def schedule_course(courses) do
    courses
    |> Enum.sort_by(fn [_, d] -> d end)
    |> Enum.reduce({0, Queue.new()}, fn [duration, lastDay], {total, q} ->
      total = total + duration
      q = Queue.push(q, duration)

      if total > lastDay do
        {v, q} = Queue.pop(q)
        {total - v, q}
      else
        {total, q}
      end
    end)
    |> elem(1)
    |> Queue.size()
  end
end
