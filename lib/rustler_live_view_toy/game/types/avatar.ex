defmodule RustlerLiveViewToy.Game.Avatar do
  @moduledoc """
  A character or object in the game world.
  """
  use Ecto.Schema
  import Ecto.Changeset

  embedded_schema do
    field :key, :string
    field :kind, :string, default: "entity"
    field :message, :string, default: ""
    field :speed, :integer, default: 0
    field :r, :integer, default: 0
    field :g, :integer, default: 0
    field :b, :integer, default: 0
    field :x, :integer, default: 0
    field :y, :integer, default: 0
    field :x_velocity, :integer, default: 0
    field :y_velocity, :integer, default: 0

    embeds_one(:position, Position)
    embeds_one(:sprite, Sprite)
  end

  @cast_params ~w(
    key
    kind
    message
    speed
    r
    g
    b
    x
    y
    x_velocity
    y_velocity
  )a

  def changeset(%__MODULE__{} = avatar, params \\ %{}) do
    avatar
    |> cast(params, @cast_params)
  end
end
