defmodule RustlerLiveViewToy.Game.Session do
  use TypedStruct
  alias RustlerLiveViewToy.Game.{Avatar, Entity}

  @typedoc """
  The global state of a game session.
  """
  typedstruct do
    field :avatars, %{required(binary()) => Avatar.t()}, default: %{}
    field :entities, %{required(binary()) => Entity.t()}, default: %{}
    field :reset_at, NaiveDateTime.t()
  end
end
