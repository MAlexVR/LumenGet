import { describe, expect, it } from "vitest";
import { downloaderErrorKey, initialDownloader, reduceDownloader } from "./downloaderUpdate";

describe("reduceDownloader", () => {
  it("starts idle and moves to checking when the update starts", () => {
    expect(initialDownloader).toEqual({ step: "idle" });
    expect(reduceDownloader(initialDownloader, { type: "start" })).toEqual({ step: "checking" });
  });

  it("ignores a second start while an update is running", () => {
    const running = reduceDownloader(initialDownloader, { type: "start" });
    expect(reduceDownloader(running, { type: "start" })).toBe(running);
  });

  it("reports a new version or that the installed one is already the latest", () => {
    const running = reduceDownloader(initialDownloader, { type: "start" });
    expect(reduceDownloader(running, { type: "done", status: "updated", version: "2026.10.01" })).toEqual({
      step: "updated",
      version: "2026.10.01",
    });
    expect(reduceDownloader(running, { type: "done", status: "current", version: null })).toEqual({
      step: "current",
      version: null,
    });
  });

  it("keeps the error code so the screen can say what to do", () => {
    const running = reduceDownloader(initialDownloader, { type: "start" });
    expect(reduceDownloader(running, { type: "fail", code: "failed" })).toEqual({ step: "error", code: "failed" });
  });

  it("allows another try after an error", () => {
    const failed = reduceDownloader(reduceDownloader(initialDownloader, { type: "start" }), { type: "fail", code: "failed" });
    expect(reduceDownloader(failed, { type: "start" })).toEqual({ step: "checking" });
  });
});

describe("downloaderErrorKey", () => {
  it("maps every backend code to a message that says what to do", () => {
    expect(downloaderErrorKey("failed")).toBe("dlu_err_failed");
    expect(downloaderErrorKey("unsupported")).toBe("dlu_err_unsupported");
    expect(downloaderErrorKey("something-new")).toBe("dlu_err_failed");
  });
});
