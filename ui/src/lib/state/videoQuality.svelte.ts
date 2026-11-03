// The maximum video height of course downloads, shared by every platform flow
// and remembered between sessions.
import { loadVideoQuality, maxHeight, saveVideoQuality, type VideoQuality } from "../logic/videoQuality";
import { app } from "./app.svelte";

const storage = (): Storage | null => {
  try {
    return localStorage;
  } catch {
    return null;
  }
};

class VideoQualityStore {
  quality = $state<VideoQuality>(loadVideoQuality(storage()));

  /** Tells the backend the current choice (it starts at 720p on every launch). */
  apply(): void {
    void app.api?.setDownloadQuality(maxHeight(this.quality)).catch(() => {});
  }

  set(quality: VideoQuality): void {
    this.quality = quality;
    saveVideoQuality(storage(), quality);
    this.apply();
  }
}

export const videoQuality = new VideoQualityStore();
