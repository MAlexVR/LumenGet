<script lang="ts">
  // Maximum video height of course downloads, applied to every platform.
  import { onMount } from "svelte";
  import { t } from "../../lib/i18n/index.svelte";
  import { VIDEO_QUALITIES } from "../../lib/logic/videoQuality";
  import { videoQuality } from "../../lib/state/videoQuality.svelte";

  const uid = $props.id();
  // The backend starts at 720p: send the remembered choice on first sight.
  onMount(() => videoQuality.apply());
</script>

<section class="quality" aria-labelledby="vq-{uid}">
  <h2 id="vq-{uid}" class="eyebrow">{t("vq_title")}</h2>
  <div class="segmented" role="radiogroup" aria-labelledby="vq-{uid}">
    {#each VIDEO_QUALITIES as q (q)}
      <button type="button" role="radio" aria-checked={videoQuality.quality === q} onclick={() => videoQuality.set(q)}>
        {q === "best" ? t("vq_best") : `${q}p`}
      </button>
    {/each}
  </div>
  <p class="help">{t(videoQuality.quality === "best" ? "vq_help_best" : "vq_help")}</p>
</section>

<style>
  .quality {
    display: grid;
    gap: var(--s-2);
  }
  .segmented {
    display: flex;
    flex-wrap: wrap;
    justify-self: start;
    border: 1px solid var(--line);
    border-radius: var(--r-2);
    padding: 2px;
  }
  .segmented button {
    min-height: 34px;
    padding: 0 var(--s-4);
    border-radius: var(--r-1);
    color: var(--text-muted);
    font-size: var(--fs-sm);
  }
  .segmented button[aria-checked="true"] {
    background: var(--surface-3);
    color: var(--text);
  }
  .help {
    font-size: var(--fs-sm);
    color: var(--text-muted);
    max-width: 62ch;
  }
</style>
