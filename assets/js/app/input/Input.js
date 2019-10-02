export const Input = {
  mouseButton(e) {
    switch (e.buttons) {
      case 0:
        return [];
      case 1:
      case 5:
        return ["left"];
      case 2:
      case 6:
        return ["right"];
      case 3:
      case 7:
        return ["left", "right"];
      default:
        return [];
    }
  }
};
