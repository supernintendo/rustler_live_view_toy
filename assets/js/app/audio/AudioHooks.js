import { Audio } from "./Audio";

export const AudioHooks = {
  PlaySound: {
    updated() {
      Audio.playSound();
    }
  }
};
