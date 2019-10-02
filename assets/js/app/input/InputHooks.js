import { Input } from "./Input";

export const InputHooks = {
  MouseDown: {
    mounted() {
      document.addEventListener("mousedown", e => {
        this.pushEvent("mousedown", {
          button: Input.mouseButton(e),
          x: e.clientX,
          y: e.clientY
        });
        e.preventDefault();
      });
    }
  },
  MouseUp: {
    mounted() {
      document.addEventListener("mouseup", e => {
        this.pushEvent("mouseup", {
          buttons: Input.mouseButton(e),
          x: e.clientX,
          y: e.clientY
        });
        e.preventDefault();
      });
    }
  }
};
