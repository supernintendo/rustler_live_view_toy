defmodule RustlerLiveViewToy.Supervisor.Child do
  use TypedStruct

  @type child_type :: :worker | :supervisor

  typedstruct do
    field :id, term()
    field :module, module(), enforce: true
    field :start_args, [term()], default: []
    field :type, child_type(), default: :worker
  end
end
