defmodule RustlerLiveViewToy.Engine.Config do
  use TypedStruct

  typedstruct do
    field :master_key, String.t()
    field :tcp_port, integer(), default: 6142
  end
end
