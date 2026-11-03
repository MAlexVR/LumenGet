// State of "update the downloader" (the yt-dlp bundled with the Android build).

export type DownloaderState =
  | { step: "idle" }
  | { step: "checking" }
  | { step: "updated"; version: string | null }
  | { step: "current"; version: string | null }
  | { step: "error"; code: string };

export type DownloaderAction =
  | { type: "start" }
  | { type: "done"; status: "updated" | "current"; version: string | null }
  | { type: "fail"; code: string };

export const initialDownloader: DownloaderState = { step: "idle" };

export function reduceDownloader(state: DownloaderState, action: DownloaderAction): DownloaderState {
  switch (action.type) {
    case "start":
      return state.step === "checking" ? state : { step: "checking" };
    case "done":
      return { step: action.status, version: action.version };
    case "fail":
      return { step: "error", code: action.code };
  }
}

export type DownloaderErrorKey = "dlu_err_failed" | "dlu_err_unsupported";

export function downloaderErrorKey(code: string): DownloaderErrorKey {
  return code === "unsupported" ? "dlu_err_unsupported" : "dlu_err_failed";
}
