defmodule RustlerLiveViewToyWeb.GraphicsLive do
  use Phoenix.LiveView

  def render(assigns) do
    ~L"""
    <div class="level">
      <div class="stage">
        <%= for {_key, avatar} <- Map.get(assigns, :avatars, %{}) do %>
          <span
            class="avatar"
            style="
              height: 32px;
              margin-left: 16px;
              margin-top: 16px;
              background: rgba(<%= avatar.r %>, <%= avatar.g %>, <%= avatar.b %>);
              left: <%= avatar.x %>%;
              top: <%= avatar.y %>%;
              width: 32px;
              z-index: 100;">
              <span
                class="message"
                style="color: rgba(<%= avatar.r %>, <%= avatar.g %>, <%= avatar.b %>);">
                  <%= avatar.message %>
              </span>
          </span>
        <% end %>
      </div>
    </div>
    """
  end
end
