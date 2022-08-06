defmodule Solution do
  @spec poor_pigs(buckets :: integer, minutes_to_die :: integer, minutes_to_test :: integer) ::
          integer
  def poor_pigs(buckets, minutes_to_die, minutes_to_test) do
    t = div(minutes_to_test, minutes_to_die) + 1
    aux(t, 0, buckets)
  end

  def aux(t, x, buckets) do
    if Integer.pow(t, x) >= buckets do
      x
    else
      aux(t, x + 1, buckets)
    end
  end
end
