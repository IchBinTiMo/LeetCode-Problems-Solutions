defmodule Solution do
  @spec get_last_moment(n :: integer, left :: [integer], right :: [integer]) :: integer
  def get_last_moment(n, left, right) do
    case Enum.empty?(left) do
        true -> 0
        false -> Enum.max(left)
        end
    |> (&(case Enum.empty?(right) do
        true -> [&1, 0]
        false -> [&1, n - Enum.min(right)]
        end)).()
    |> Enum.max
  end
end
