<script lang="ts">
  // Choose lessons of one course and watch each one download: the ticked
  // lessons are the only ones downloaded, and the one running shows its own
  // progress bar.
  import { onMount } from "svelte";
  import Icon from "../../lib/design/Icon.svelte";
  import ProgressLine from "../../lib/components/ProgressLine.svelte";
  import { t } from "../../lib/i18n/index.svelte";
  import type { DownloadEntry } from "../../lib/logic/downloads";
  import {
    groupByModule,
    lessonStates,
    selectedOrders,
    selectionAfterToggle,
    selectionAfterToggleModule,
    type ModuleGroup,
    type PickableLesson,
  } from "../../lib/logic/lessonPicker";
  import { displayTitle } from "../../lib/logic/title";
  import type { DownloadItemSession } from "../../lib/state/downloads.svelte";

  interface Props {
    session: DownloadItemSession;
    itemKey: string;
    name: string;
    /** The download of this course that is running, if any. */
    entry: DownloadEntry | undefined;
    onclose: () => void;
  }
  let { session, itemKey, name, entry, onclose }: Props = $props();

  let lessons = $state<PickableLesson[]>([]);
  let loading = $state(true);
  let failed = $state(false);
  let selection = $state<ReadonlySet<number>>(new Set());
  /** The lessons of the download started from here, to tell which one is running. */
  let started = $state<number[] | null>(null);

  const groups = $derived(groupByModule(lessons));
  const running = $derived(entry?.status === "running");
  const locked = $derived(running || !!session.starting[itemKey]);
  const states = $derived(started ? lessonStates(started, entry?.completed ?? 0, running) : null);
  const count = $derived(selection.size);

  async function load() {
    loading = true;
    failed = false;
    try {
      lessons = (await session.lessons?.(itemKey)) ?? [];
    } catch {
      failed = true;
    } finally {
      loading = false;
    }
  }
  onMount(() => void load());

  const chosenIn = (group: ModuleGroup) => group.lessons.filter((l) => selection.has(l.order)).length;

  async function download() {
    const orders = selectedOrders(selection);
    if (!orders.length || !session.startLessons) return;
    started = orders;
    await session.startLessons(itemKey, orders);
    // A start that failed leaves the error note in the row: allow picking again.
    if (session.startErrors[itemKey]) started = null;
  }

  const percent = $derived(Math.round(Math.max(0, Math.min(100, entry?.percent ?? 0))));
</script>

<section class="picker" aria-label={t("lp_title", { title: displayTitle(name) })}>
  {#if loading}
    <p class="muted" role="status">{t("lp_loading")}</p>
  {:else if failed}
    <p class="muted" role="alert">{t("lp_error")}</p>
    <div><button type="button" class="btn btn-quiet" onclick={load}>{t("lp_retry")}</button></div>
  {:else}
    <header>
      <p class="chosen num" aria-live="polite">{t("lp_chosen", { n: count, total: lessons.length })}</p>
      <div class="bulk">
        <button type="button" class="btn link" disabled={locked} onclick={() => (selection = new Set(lessons.map((l) => l.order)))}>{t("lp_all")}</button>
        <button type="button" class="btn link" disabled={locked} onclick={() => (selection = new Set())}>{t("lp_none")}</button>
      </div>
    </header>

    <div class="groups">
      {#each groups as group, gi (gi)}
        {@const chosen = chosenIn(group)}
        <details>
          <summary>
            <input
              type="checkbox"
              aria-label={group.module || t("lp_no_module")}
              checked={chosen === group.lessons.length}
              indeterminate={chosen > 0 && chosen < group.lessons.length}
              disabled={locked}
              onclick={(e) => e.stopPropagation()}
              onchange={() => (selection = selectionAfterToggleModule(selection, group))}
            />
            <span class="module">{displayTitle(group.module) || t("lp_no_module")}</span>
            <span class="num count">{t("lp_module_count", { n: chosen, total: group.lessons.length })}</span>
          </summary>
          <ul>
            {#each group.lessons as lesson (lesson.order)}
              {@const state = states?.get(lesson.order)}
              <li data-state={state}>
                <label>
                  <input
                    type="checkbox"
                    checked={selection.has(lesson.order)}
                    disabled={locked}
                    onchange={() => (selection = selectionAfterToggle(selection, lesson.order))}
                  />
                  <span class="n num">{lesson.order + 1}</span>
                  <span class="title">{displayTitle(lesson.title)}</span>
                </label>
                {#if state === "done"}
                  <span class="state ok"><Icon name="check" size={14} />{t("lp_state_done")}</span>
                {:else if state === "pending"}
                  <span class="state">{t("lp_state_pending")}</span>
                {:else if state === "current"}
                  <span class="state now num">{percent > 0 ? t("lp_state_current", { p: percent }) : t("lp_state_current_start")}</span>
                  <div class="bar">
                    <ProgressLine value={percent} thickness={3} label={t("lp_title", { title: displayTitle(lesson.title) })} />
                  </div>
                {/if}
              </li>
            {/each}
          </ul>
        </details>
      {/each}
    </div>

    <footer>
      <button type="button" class="btn btn-primary" disabled={!count || locked} onclick={download}>
        <Icon name="download" size={16} />{t(count === 1 ? "lp_download_one" : "lp_download_n", { n: count })}
      </button>
      <button type="button" class="btn link" onclick={onclose}>{t("lp_hide")}</button>
    </footer>
    <p class="hint">{t("lp_pick_hint")}</p>
  {/if}
</section>

<style>
  .picker {
    display: grid;
    gap: var(--s-4);
    border-top: 1px solid var(--line);
    padding-top: var(--s-4);
  }
  header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--s-4);
  }
  .chosen {
    color: var(--text);
    font-size: var(--fs-sm);
  }
  .bulk,
  footer {
    display: flex;
    align-items: center;
    gap: var(--s-3);
    flex-wrap: wrap;
  }
  .groups {
    display: grid;
    border-top: 1px solid var(--line-soft);
    max-height: 60vh;
    overflow-y: auto;
  }
  details {
    border-bottom: 1px solid var(--line-soft);
  }
  summary {
    display: flex;
    align-items: center;
    gap: var(--s-3);
    padding: var(--s-3) 0;
    cursor: pointer;
    min-height: 44px;
  }
  .module {
    flex: 1;
    min-width: 0;
    color: var(--text);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .count {
    color: var(--text-muted);
    font-size: var(--fs-xs);
  }
  ul {
    display: grid;
    padding: 0 0 var(--s-2) var(--s-6);
  }
  li {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    align-items: center;
    column-gap: var(--s-4);
    row-gap: var(--s-1);
    min-height: 40px;
  }
  li label {
    display: flex;
    align-items: center;
    gap: var(--s-3);
    min-width: 0;
    min-height: 40px;
    cursor: pointer;
  }
  .n {
    color: var(--text-faint);
    font-size: var(--fs-xs);
    min-width: 3ch;
    text-align: right;
  }
  .title {
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    color: var(--text-muted);
  }
  li[data-state="current"] .title {
    color: var(--text);
  }
  .state {
    display: inline-flex;
    align-items: center;
    gap: var(--s-1);
    font-size: var(--fs-xs);
    color: var(--text-muted);
  }
  .state.ok {
    color: var(--ok);
  }
  .state.now {
    color: var(--lumen);
  }
  .bar {
    grid-column: 1 / -1;
  }
  input[type="checkbox"] {
    accent-color: var(--lumen);
    width: 18px;
    height: 18px;
    flex: none;
  }
  @media (pointer: coarse) {
    input[type="checkbox"] {
      width: 24px;
      height: 24px;
    }
    li,
    li label {
      min-height: 48px;
    }
  }
  .muted,
  .hint {
    color: var(--text-muted);
    font-size: var(--fs-sm);
  }
  .hint {
    font-size: var(--fs-xs);
  }
</style>
