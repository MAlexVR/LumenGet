<script lang="ts">
  import { onMount } from "svelte";
  import Icon from "../../lib/design/Icon.svelte";
  import { t } from "../../lib/i18n/index.svelte";
  import { musicAttr, percentToVolume, trackLabel, volumeToPercent } from "../../lib/logic/music";
  import { app } from "../../lib/state/app.svelte";
  import { command, music, setVolumeLocally, watchMusic } from "../../lib/state/music.svelte";

  let { idle }: { idle: boolean } = $props();

  let open = $state(false);
  const now = $derived(music.now);
  const label = $derived(now ? trackLabel(now) : null);

  onMount(() => watchMusic());

  function setVolume(e: Event) {
    const volume = percentToVolume(Number((e.currentTarget as HTMLInputElement).value));
    setVolumeLocally(volume);
    void command(() => app.api?.mediaSetVolume(volume) ?? Promise.resolve());
  }
</script>

{#if now && label}
  <div class="music" class:idle class:open {...{ [musicAttr]: "" }} role="group" aria-label={t("music_label")}>
    {#if open}
      <div class="panel">
        <div class="track">
          <span class="title" title={label.title}>{label.title}</span>
          {#if label.artist}<span class="artist" title={label.artist}>{label.artist}</span>{/if}
        </div>
        <div class="buttons">
          <button
            type="button"
            class="btn-ico"
            aria-label={t("music_prev")}
            title={t("music_prev")}
            disabled={!now.canPrevious}
            onclick={() => command(() => app.api?.mediaPrevious() ?? Promise.resolve())}
          >
            <Icon name="prev" size={18} />
          </button>
          <button
            type="button"
            class="btn-ico main"
            aria-label={t(now.playing ? "music_pause" : "music_play")}
            title={t(now.playing ? "music_pause" : "music_play")}
            onclick={() => command(() => app.api?.mediaPlayPause() ?? Promise.resolve())}
          >
            <Icon name={now.playing ? "pause" : "play"} size={18} />
          </button>
          <button
            type="button"
            class="btn-ico"
            aria-label={t("music_next")}
            title={t("music_next")}
            disabled={!now.canNext}
            onclick={() => command(() => app.api?.mediaNext() ?? Promise.resolve())}
          >
            <Icon name="next" size={18} />
          </button>
        </div>
        {#if now.volume !== null}
          <input
            class="volume"
            type="range"
            min="0"
            max="100"
            step="5"
            value={volumeToPercent(now.volume)}
            aria-label={t("music_volume")}
            oninput={setVolume}
          />
        {/if}
      </div>
    {/if}
    <button
      type="button"
      class="toggle"
      class:playing={now.playing}
      aria-expanded={open}
      aria-label={t(open ? "music_close" : "music_open")}
      title="{label.title}{label.artist ? ` — ${label.artist}` : ''}"
      onclick={() => (open = !open)}
    >
      <Icon name="music" size={18} />
    </button>
  </div>
{/if}

<style>
  .music {
    position: absolute;
    right: var(--s-4);
    bottom: 84px;
    z-index: 4;
    display: grid;
    justify-items: end;
    gap: var(--s-2);
    transition: opacity var(--dur-3) var(--ease-out);
  }
  .music.idle:not(.open) {
    opacity: 0.55;
  }
  .music:hover,
  .music:focus-within {
    opacity: 1;
  }
  .toggle,
  .panel {
    border: 1px solid var(--line);
    background: color-mix(in srgb, var(--surface-1) 96%, transparent);
    backdrop-filter: blur(8px);
    color: var(--text);
  }
  .toggle {
    display: grid;
    place-items: center;
    width: 40px;
    height: 40px;
    border-radius: 50%;
    cursor: pointer;
    color: var(--text-muted);
  }
  .toggle:hover,
  .toggle:focus-visible,
  .toggle.playing {
    color: var(--lumen);
  }
  .toggle.playing {
    box-shadow: 0 0 14px color-mix(in srgb, var(--lumen) 28%, transparent);
  }
  .panel {
    display: grid;
    gap: var(--s-3);
    width: 240px;
    padding: var(--s-3) var(--s-4);
    border-radius: var(--r-3);
  }
  .track {
    display: grid;
    min-width: 0;
  }
  .title,
  .artist {
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
  }
  .title {
    font-size: var(--fs-sm);
    font-weight: 500;
  }
  .artist {
    font-size: var(--fs-xs);
    color: var(--text-muted);
  }
  .buttons {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: var(--s-3);
  }
  .btn-ico {
    display: grid;
    place-items: center;
    width: 36px;
    height: 36px;
    border: 0;
    border-radius: 50%;
    background: transparent;
    color: var(--text);
    cursor: pointer;
  }
  .btn-ico:hover:not(:disabled),
  .btn-ico:focus-visible {
    background: color-mix(in srgb, var(--text) 12%, transparent);
  }
  .btn-ico:disabled {
    color: var(--text-faint);
    cursor: default;
  }
  .btn-ico.main {
    border: 1px solid var(--line);
  }
  .volume {
    width: 100%;
    accent-color: var(--lumen);
  }
  @media (prefers-reduced-motion: reduce) {
    .music {
      transition: none;
    }
  }
</style>
