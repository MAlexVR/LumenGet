<script lang="ts">
  // Ajustes › Descargador (Android): the version of the bundled yt-dlp and a
  // button that fetches the latest release.
  import { onMount } from "svelte";
  import { t } from "../../lib/i18n/index.svelte";
  import {
    downloaderErrorKey,
    initialDownloader,
    reduceDownloader,
    type DownloaderAction,
    type DownloaderState,
  } from "../../lib/logic/downloaderUpdate";
  import { app } from "../../lib/state/app.svelte";

  const uid = $props.id();
  let status = $state<DownloaderState>(initialDownloader);
  let version = $state<string | null>(null);
  const dispatch = (a: DownloaderAction) => (status = reduceDownloader(status, a));

  onMount(() => {
    void app.api
      ?.downloaderVersion()
      .then((info) => (version = info.version))
      .catch(() => {});
  });

  async function update() {
    if (status.step === "checking") return;
    dispatch({ type: "start" });
    try {
      const result = await app.api!.downloaderUpdate();
      version = result.version ?? version;
      dispatch({ type: "done", status: result.status, version: result.version });
    } catch (e) {
      dispatch({ type: "fail", code: (e as { code?: string })?.code ?? "failed" });
    }
  }
</script>

<section class="row" aria-labelledby="dlu-h-{uid}">
  <h2 id="dlu-h-{uid}" class="eyebrow">{t("dlu_title")}</h2>
  <div class="body">
    <p class="hint">{t("dlu_hint")}</p>
    <p class="fact">
      <span class="label">{t("dlu_version")}</span>
      <span class="num">{version ?? t("dlu_version_unknown")}</span>
    </p>
    <div class="actions">
      <button type="button" class="btn btn-quiet" onclick={update} disabled={status.step === "checking"}>
        {t("dlu_update")}
      </button>
    </div>
    <p class="result" role="status" aria-live="polite">
      {#if status.step === "checking"}
        {t("dlu_checking")}
      {:else if status.step === "updated"}
        {t("dlu_updated", { v: status.version ?? "" })}
      {:else if status.step === "current"}
        {t("dlu_current")}
      {:else if status.step === "error"}
        {t(downloaderErrorKey(status.code))}
      {/if}
    </p>
  </div>
</section>

<style>
  .row {
    display: grid;
    grid-template-columns: minmax(140px, 220px) 1fr;
    gap: var(--s-6);
    padding: var(--s-6) 0;
    border-top: 1px solid var(--line);
    align-items: start;
  }
  .row h2 {
    font-family: var(--font-ui);
    padding-top: var(--s-2);
  }
  .body {
    display: grid;
    gap: var(--s-4);
    min-width: 0;
  }
  .hint,
  .result {
    color: var(--text-muted);
  }
  .label {
    color: var(--text-muted);
    margin-right: var(--s-2);
  }
  .result:empty {
    display: none;
  }
  @media (max-width: 640px) {
    .row {
      grid-template-columns: 1fr;
      gap: var(--s-3);
    }
  }
</style>
