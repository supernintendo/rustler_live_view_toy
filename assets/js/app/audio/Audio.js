import { Howl, Howler } from "howler";

export const Audio = {
  playSound() {
    var sound = new Howl({
      src: ["audio/test_sound.wav"]
    });
    sound.play();
  }
};
