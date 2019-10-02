defmodule RustlerLiveViewToy.Supervisor.Config do
  use TypedStruct

  typedstruct do
    field :child_defs, [RustlerLiveViewToy.Supervisor.Child.t()]
    field :children, [map()], default: []
    field :name, Supervisor.name(), enforce: true
    field :strategy, Supervisor.strategy(), default: :one_for_one
  end
end
