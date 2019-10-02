import { AudioHooks } from "./audio/AudioHooks";
import { InputHooks } from "./input/InputHooks";

export const Hooks = {
  ...AudioHooks,
  ...InputHooks
};
