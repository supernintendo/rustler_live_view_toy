defmodule RustlerLiveViewToy.Game.Entity do
  use TypedStruct

  @typedoc """
  A human player playing the game.
  """
  typedstruct do
    field :high_score, integer(), default: 0
    field :id, String.t(), enforce: true
    field :name, String.t()
    field :process, pid()
    field :score, integer(), default: 0
  end
end
