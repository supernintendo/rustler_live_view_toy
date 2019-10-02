// We need to import the CSS so that webpack will load it.
// The MiniCssExtractPlugin is used to separate it out into
// its own CSS file.
import css from "../css/app.css";

// Import dependencies
import "phoenix_html";
import { Socket } from "phoenix";
import { LiveSocket } from "phoenix_live_view";
import { Hooks } from "./app/Hooks";

let liveSocket = new LiveSocket("/live", Socket, { hooks: Hooks });

liveSocket.connect();
