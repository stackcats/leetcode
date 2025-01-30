defmodule Solution do
  @spec fraction_addition(expression :: String.t()) :: String.t()
  def fraction_addition(expression) do
    expr = if expression =~ ~r/^\d/, do: "+" <> expression, else: expression

    Regex.scan(~r/(\d+\/\d+|\+|\-)/, expr)
    |> Enum.map(&List.first/1)
    |> Enum.chunk_every(2)
    |> Enum.reduce({0, 1}, fn [op, s], acc ->
      [c, d] =
        s
        |> String.split("/")
        |> Enum.map(&String.to_integer/1)

      case op do
        "+" -> calc(acc, {c, d}, &Kernel.+/2)
        "-" -> calc(acc, {c, d}, &Kernel.-/2)
      end
    end)
    |> normalise()
    |> then(fn {a, b} -> "#{a}/#{b}" end)
  end

  def calc({a, b}, {c, d}, op) do
    {op.(a * d, c * b), b * d}
  end

  def normalise({0, _}), do: {0, 1}

  def normalise({a, b}) do
    sign = if a * b < 0, do: -1, else: 1
    a = abs(a)
    b = abs(b)
    g = Integer.gcd(a, b)
    {sign * div(a, g), div(b, g)}
  end
end
