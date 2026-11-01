<script lang="ts">
  import { onMount } from "svelte";
  import Icon from "../../lib/design/Icon.svelte";
  import { t } from "../../lib/i18n/index.svelte";
  import {
    clampSeek,
    displayedPosition,
    muteTarget,
    musicAttr,
    nextRepeat,
    percentToVolume,
    progressFraction,
    trackLabel,
    volumeToPercent,
  } from "../../lib/logic/music";
  import { formatTime } from "../../lib/logic/time";
  import { app } from "../../lib/state/app.svelte";
  import { command, music, setPositionLocally, setVolumeLocally, watchMusic } from "../../lib/state/music.svelte";

  let { idle }: { idle: boolean } = $props();

  let open = $state(false);
  /** Cover that failed to load: shown as the note until the song changes. */
  let brokenCover = $state<string | null>(null);
  /** The last audible volume, brought back when the speaker button unmutes. */
  let beforeMute = $state(0.5);
  let tick = $state(performance.now());
  /** Where the slider is while it is being dragged, before the jump is made. */
  let scrub = $state<number | null>(null);

  const now = $derived(music.now);
  const label = $derived(now ? trackLabel(now) : null);
  const position = $derived(
    now ? displayedPosition(now.positionSecs, now.lengthSecs, now.playing, (tick - music.readAt) / 1000) : 0,
  );
  const shown = $derived(scrub ?? position);
  const fraction = $derived(now ? progressFraction(shown, now.lengthSecs) : 0);
  const repeatLabel = $derived(now ? t(`music_repeat_${now.repeat}` as const) : "");

  onMount(() => watchMusic());

  $effect(() => {
    if (now?.volume) beforeMute = now.volume;
  });

  // The bar moves between two readings of Spotify, only while it can be seen moving.
  $effect(() => {
    if (!open || !now?.playing) return;
    tick = performance.now();
    const timer = setInterval(() => (tick = performance.now()), 500);
    return () => clearInterval(timer);
  });

  const run = (what: () => Promise<void> | undefined) => command(() => what() ?? Promise.resolve());

  function setVolume(e: Event) {
    const volume = percentToVolume(Number((e.currentTarget as HTMLInputElement).value));
    setVolumeLocally(volume);
    void run(() => app.api?.mediaSetVolume(volume));
  }

  function seek(e: Event) {
    if (!now) return;
    const seconds = clampSeek(Number((e.currentTarget as HTMLInputElement).value), now.lengthSecs);
    setPositionLocally(seconds);
    void run(() => app.api?.mediaSeek(seconds)).finally(() => (scrub = null));
  }

  function toggleMute() {
    if (!now) return;
    const volume = muteTarget(now.volume, beforeMute);
    setVolumeLocally(volume);
    void run(() => app.api?.mediaSetVolume(volume));
  }
</script>

{#if now && label}
  <div class="music" class:idle class:open {...{ [musicAttr]: "" }} role="group" aria-label={t("music_label")}>
    {#if open}
      <div class="panel">
        <div class="head">
          <div class="cover">
            {#if now.artUrl && now.artUrl !== brokenCover}
              <img
                src={now.artUrl}
                alt={t("music_cover")}
                referrerpolicy="no-referrer"
                onerror={() => (brokenCover = now.artUrl)}
              />
            {:else}
              <Icon name="music" size={24} />
            {/if}
          </div>
          <div class="track">
            <span class="title">{label.title}</span>
            {#if label.artist}<span class="artist">{label.artist}</span>{/if}
          </div>
        </div>

        <div class="row">
          <span class="clock num">{formatTime(shown)}</span>
          <input
            class="slider"
            type="range"
            min="0"
            max={now.lengthSecs ?? 0}
            step="1"
            value={Math.round(shown)}
            disabled={!now.canSeek || now.lengthSecs === null}
            aria-label={t("music_progress")}
            aria-valuetext="{formatTime(shown)} / {now.lengthSecs === null ? '–:––' : formatTime(now.lengthSecs)}"
            style:--fill="{fraction * 100}%"
            style:--color="var(--text)"
            oninput={(e) => (scrub = Number(e.currentTarget.value))}
            onchange={seek}
          />
          <span class="clock num end">{now.lengthSecs === null ? "–:––" : formatTime(now.lengthSecs)}</span>
        </div>

        <div class="buttons">
          <button
            type="button"
            class="btn-ico"
            class:on={now.shuffle}
            aria-label={t("music_shuffle")}
            title={t("music_shuffle")}
            aria-pressed={now.shuffle}
            onclick={() => run(() => app.api?.mediaSetShuffle(!now.shuffle))}
          >
            <Icon name="shuffle" size={18} />
          </button>
          <button
            type="button"
            class="btn-ico"
            aria-label={t("music_prev")}
            title={t("music_prev")}
            disabled={!now.canPrevious}
            onclick={() => run(() => app.api?.mediaPrevious())}
          >
            <Icon name="prev" size={18} />
          </button>
          <button
            type="button"
            class="btn-ico main"
            aria-label={t(now.playing ? "music_pause" : "music_play")}
            title={t(now.playing ? "music_pause" : "music_play")}
            onclick={() => run(() => app.api?.mediaPlayPause())}
          >
            <Icon name={now.playing ? "pause" : "play"} size={20} />
          </button>
          <button
            type="button"
            class="btn-ico"
            aria-label={t("music_next")}
            title={t("music_next")}
            disabled={!now.canNext}
            onclick={() => run(() => app.api?.mediaNext())}
          >
            <Icon name="next" size={18} />
          </button>
          <button
            type="button"
            class="btn-ico"
            class:on={now.repeat !== "none"}
            aria-label={repeatLabel}
            title={repeatLabel}
            aria-pressed={now.repeat !== "none"}
            onclick={() => run(() => app.api?.mediaSetRepeat(nextRepeat(now.repeat)))}
          >
            <Icon name={now.repeat === "track" ? "repeatOne" : "loop"} size={18} />
          </button>
        </div>

        {#if now.volume !== null}
          <div class="row">
            <button
              type="button"
              class="btn-ico small"
              aria-label={t(now.volume > 0 ? "music_mute" : "music_unmute")}
              title={t(now.volume > 0 ? "music_mute" : "music_unmute")}
              onclick={toggleMute}
            >
              <Icon name={now.volume > 0 ? "volume" : "mute"} size={18} />
            </button>
            <input
              class="slider"
              type="range"
              min="0"
              max="100"
              step="5"
              value={volumeToPercent(now.volume)}
              aria-label={t("music_volume")}
              style:--fill="{volumeToPercent(now.volume)}%"
              style:--color="var(--lumen)"
              oninput={setVolume}
            />
            <span aria-hidden="true"></span>
          </div>
        {/if}
      </div>
    {/if}
    <button
      type="button"
      class="toggle"
      class:playing={now.playing}
      aria-expanded={open}
      aria-label={t(open ? "music_close" : "music_open")}
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
    gap: var(--s-2);
    width: min(320px, 80vw);
    padding: var(--s-3) var(--s-4);
    border-radius: var(--r-3);
  }
  .head {
    display: flex;
    align-items: center;
    gap: var(--s-3);
    min-width: 0;
  }
  .cover {
    display: grid;
    place-items: center;
    flex: none;
    width: 48px;
    height: 48px;
    overflow: hidden;
    border-radius: var(--r-2);
    background: var(--surface-2);
    color: var(--text-muted);
  }
  .cover img {
    width: 100%;
    height: 100%;
    object-fit: cover;
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
    font-size: var(--fs-md);
    font-weight: 600;
  }
  .artist {
    font-size: var(--fs-sm);
    color: var(--text-muted);
  }
  /* The song bar and the volume bar share these columns, so they are as long as each other. */
  .row {
    display: grid;
    grid-template-columns: 2.75rem minmax(0, 1fr) 2.75rem;
    align-items: center;
    column-gap: var(--s-2);
  }
  .clock {
    font-size: var(--fs-xs);
    color: var(--text-muted);
  }
  .clock.end {
    text-align: right;
  }
  .slider {
    -webkit-appearance: none;
    appearance: none;
    width: 100%;
    height: 16px;
    margin: 0;
    background: transparent;
    cursor: pointer;
  }
  .slider:disabled {
    opacity: 0.5;
    cursor: default;
  }
  .slider::-webkit-slider-runnable-track {
    height: 4px;
    border-radius: 2px;
    background: linear-gradient(to right, var(--color) var(--fill), var(--line) var(--fill));
  }
  .slider::-moz-range-track {
    height: 4px;
    border-radius: 2px;
    background: linear-gradient(to right, var(--color) var(--fill), var(--line) var(--fill));
  }
  .slider::-webkit-slider-thumb {
    -webkit-appearance: none;
    width: 12px;
    height: 12px;
    margin-top: -4px;
    border: 0;
    border-radius: 50%;
    background: var(--text);
    opacity: 0;
  }
  .slider::-moz-range-thumb {
    width: 12px;
    height: 12px;
    border: 0;
    border-radius: 50%;
    background: var(--text);
    opacity: 0;
  }
  .slider:hover::-webkit-slider-thumb,
  .slider:focus-visible::-webkit-slider-thumb {
    opacity: 1;
  }
  .slider:hover::-moz-range-thumb,
  .slider:focus-visible::-moz-range-thumb {
    opacity: 1;
  }
  .buttons {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: var(--s-4);
  }
  .btn-ico {
    display: grid;
    place-items: center;
    width: 36px;
    height: 36px;
    border: 0;
    border-radius: 50%;
    background: transparent;
    color: var(--text-muted);
    cursor: pointer;
  }
  .btn-ico:hover:not(:disabled),
  .btn-ico:focus-visible {
    color: var(--text);
  }
  .btn-ico:disabled {
    color: var(--text-faint);
    cursor: default;
  }
  .btn-ico.on {
    color: var(--lumen);
  }
  .btn-ico.main {
    width: 40px;
    height: 40px;
    background: var(--text);
    color: var(--bg);
  }
  .btn-ico.main:hover:not(:disabled),
  .btn-ico.main:focus-visible {
    color: var(--bg);
    transform: scale(1.06);
  }
  .btn-ico.small {
    width: 28px;
    height: 28px;
  }
  @media (prefers-reduced-motion: reduce) {
    .music,
    .btn-ico.main:hover:not(:disabled) {
      transition: none;
      transform: none;
    }
  }
</style>
