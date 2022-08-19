defmodule Solution do
  @spec mirror_reflection(p :: integer, q :: integer) :: integer
  def mirror_reflection(p, q) do
    if rem(p, 2) == 0 && rem(q, 2) == 0 do
      mirror_reflection(div(p, 2), div(q, 2))
    else
      case {rem(p, 2), rem(q, 2)} do
        {1, 0} -> 0
        {1, 1} -> 1
        {0, 1} -> 2
        _ -> -1
      end
    end
  end
end
