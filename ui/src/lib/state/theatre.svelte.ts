// Theatre mode is read by the player and by the app shell (the side menu steps
// aside in it), so it lives here instead of in the player.

const KEY = "lumen.theatre";

function read(): boolean {
  try {
    return localStorage.getItem(KEY) === "1";
  } catch {
    return false;
  }
}

export const cinema = $state({ on: read() });

export function toggleCinema(): void {
  cinema.on = !cinema.on;
  try {
    localStorage.setItem(KEY, cinema.on ? "1" : "0");
  } catch {
    /* not persisted */
  }
}
