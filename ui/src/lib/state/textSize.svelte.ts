// The reader's text size for lesson content, shared by every description and
// remembered between sessions.
import { TEXT_SIZES, loadTextSize, saveTextSize, stepTextSize } from "../logic/textSize";

const storage = (): Storage | null => {
  try {
    return localStorage;
  } catch {
    return null;
  }
};

class TextSizeStore {
  px = $state(loadTextSize(storage()));

  get canShrink(): boolean {
    return this.px > TEXT_SIZES[0];
  }

  get canGrow(): boolean {
    return this.px < TEXT_SIZES[TEXT_SIZES.length - 1];
  }

  #set(px: number): void {
    this.px = px;
    saveTextSize(storage(), px);
  }

  smaller(): void {
    this.#set(stepTextSize(this.px, -1));
  }

  bigger(): void {
    this.#set(stepTextSize(this.px, 1));
  }
}

export const textSize = new TextSizeStore();
