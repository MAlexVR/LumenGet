// Touch has no hover to wake the player controls, so the first tap on a
// video whose controls faded away must only bring them back; pausing on that
// same tap would make the controls unreachable while playing.

export type VideoTap = "reveal" | "toggle";

export function videoTap(pointerType: string, controlsHidden: boolean): VideoTap {
  return pointerType === "touch" && controlsHidden ? "reveal" : "toggle";
}
