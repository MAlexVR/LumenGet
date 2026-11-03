<script lang="ts">
  // Result of a finished download: state, counts and grouped issues.
  import Icon, { type IconName } from "../../lib/design/Icon.svelte";
  import { t, tn } from "../../lib/i18n/index.svelte";
  import { app } from "../../lib/state/app.svelte";
  import type { DownloadEntry } from "../../lib/logic/downloads";
  import { issueNote, missingCounts, summarizeReport, type IssueGroup } from "../../lib/logic/report";
  import { displayTitle } from "../../lib/logic/title";
  import type { ReportEvent } from "../../lib/types";

  interface Props {
    entry: DownloadEntry;
    report: ReportEvent;
    onopen: () => void;
    onretry: () => void;
    ondismiss: () => void;
  }
  let { entry, report, onopen, onretry, ondismiss }: Props = $props();
  const uid = $props.id();
  let panel: HTMLElement | undefined = $state();
  let heading: HTMLElement | undefined = $state();
  let copied = $state<string | null>(null);

  const summary = $derived(summarizeReport(report));
  const missing = $derived(missingCounts(report));
  const missingText = $derived.by(() => {
    const parts = [
      missing.videos > 0 && t(missing.videos === 1 ? "ck_videos_one" : "ck_videos_n", { n: missing.videos }),
      missing.materials > 0 && t(missing.materials === 1 ? "ck_materials_one" : "ck_materials_n", { n: missing.materials }),
    ].filter((p): p is string => typeof p === "string");
    return parts.length ? t("ck_counts_missing", { items: parts.join(` ${t("ck_and")} `) }) : tn("dl_errors", summary.errorCount);
  });
  const ICON: Record<string, IconName> = { complete: "check", warnings: "check", errors: "alert", cancelled: "close", pending: "download" };
  /** A check that found things missing is a list of pending items, not of errors. */
  const pending = $derived(summary.checked && summary.state === "errors");
  const view = $derived(pending ? "pending" : summary.state);
  const CHECK_KINDS = { video: "ck_kind_video", video_not_resolved: "ck_kind_video", resource: "ck_kind_resource" } as const;
  const kindLabel = (kind: IssueGroup["kind"]) => (summary.checked && kind in CHECK_KINDS ? t(CHECK_KINDS[kind as keyof typeof CHECK_KINDS]) : t(`dl_kind_${kind}`));

  // Move focus to the result when it replaces the control the user was on.
  $effect(() => {
    const active = document.activeElement;
    if (!active || active === document.body || panel?.parentElement?.contains(active)) heading?.focus();
  });

  async function copy(text: string, key: string) {
    try {
      await navigator.clipboard.writeText(text);
      copied = key;
      setTimeout(() => (copied = null), 1500);
    } catch {
      const el = document.getElementById(key);
      const sel = window.getSelection();
      if (el && sel) {
        const range = document.createRange();
        range.selectNodeContents(el);
        sel.removeAllRanges();
        sel.addRange(range);
      }
    }
  }
</script>

{#snippet issues(groups: IssueGroup[], severity: "error" | "warning")}
  {#each groups as g (g.kind)}
    <details class="kind" data-severity={severity} open={severity === "error"}>
      <summary>
        <Icon name="chevronRight" size={14} />
        <span>{kindLabel(g.kind)}</span>
        <span class="count num">{g.issues.length}</span>
      </summary>
      <ul>
        {#each g.issues as issue, i (i)}
          {@const key = `url-${uid}-${severity}-${g.kind}-${i}`}
          {@const note = issueNote(issue)}
          <li>
            <p class="lesson">{issue.lesson ? displayTitle(issue.lesson) : t("dl_lesson_unknown")}</p>
            {#if note}
              <p class="msg">{t(note.message)}</p>
              <p class="hint">{t(note.hint)}</p>
            {:else if summary.checked && issue.message.startsWith("missing:")}
              <p class="msg">{t("ck_missing_item", { what: issue.message.slice("missing:".length).trim() })}</p>
            {:else}
              <p class="msg">{issue.message}</p>
            {/if}
            {#if issue.url || issue.host}
              <p class="where">
                <code id={key}>{issue.url ?? issue.host}</code>
                <button type="button" class="copy" onclick={() => copy(issue.url ?? issue.host ?? "", key)}>
                  {t(copied === key ? "dl_copied" : "dl_copy")}
                </button>
              </p>
            {/if}
          </li>
        {/each}
      </ul>
    </details>
  {/each}
{/snippet}

<section class="result" data-state={view} aria-labelledby="res-{uid}" bind:this={panel}>
  <header>
    <span class="mark" aria-hidden="true"><Icon name={ICON[view]} size={20} /></span>
    <div>
      {#if summary.checked && summary.state !== "cancelled"}
        <h4 id="res-{uid}" tabindex="-1" bind:this={heading}>{t(summary.state === "errors" ? "ck_state_missing" : "ck_state_complete")}</h4>
        <p class="counts num">
          {summary.state === "errors" ? missingText : t("ck_counts_complete", { total: summary.total })}
        </p>
      {:else}
        <h4 id="res-{uid}" tabindex="-1" bind:this={heading}>{t(`dl_state_${summary.state}`)}</h4>
        <p class="counts num">
          {t("dl_result_counts", { ok: summary.ok, total: summary.total })}
          {#if summary.errorCount}<span class="e">· {tn("dl_errors", summary.errorCount)}</span>{/if}
          {#if summary.warningCount}<span class="w">· {tn("dl_warnings", summary.warningCount)}</span>{/if}
        </p>
      {/if}
    </div>
  </header>

  {#if summary.errors.length || summary.warnings.length}
    <div class="issues">
      {@render issues(summary.errors, "error")}
      {@render issues(summary.warnings, "warning")}
    </div>
  {/if}

  <div class="actions">
    {#if app.capabilities.openFolders}
      <button type="button" class="btn btn-quiet" onclick={onopen}>
        <Icon name="file" size={16} />{t("dl_open_folder")}
      </button>
    {/if}
    {#if summary.state === "errors" || summary.state === "cancelled"}
      <button type="button" class="btn btn-quiet" onclick={onretry}>{t(summary.checked ? "ck_download_missing" : "dl_retry")}</button>
    {/if}
    <button type="button" class="btn link" onclick={ondismiss} data-id={entry.id}>{t("dl_dismiss")}</button>
  </div>
</section>

<style>
  .result {
    display: grid;
    gap: var(--s-4);
    padding: var(--s-5);
    background: var(--surface-1);
    border-radius: var(--r-3);
    border-left: 2px solid var(--line);
  }
  .result[data-state="complete"],
  .result[data-state="warnings"] {
    border-left-color: var(--ok);
  }
  .result[data-state="errors"] {
    border-left-color: var(--danger);
  }
  .result[data-state="pending"] {
    border-left-color: var(--moon);
  }
  header {
    display: flex;
    gap: var(--s-3);
    align-items: flex-start;
  }
  .mark {
    color: var(--text-muted);
    padding-top: 6px;
  }
  [data-state="complete"] .mark,
  [data-state="warnings"] .mark {
    color: var(--ok);
  }
  [data-state="errors"] .mark {
    color: var(--danger);
  }
  [data-state="pending"] .mark {
    color: var(--moon);
  }
  [data-state="pending"] .count {
    color: var(--moon);
  }
  h4 {
    font-family: var(--font-display);
    font-weight: var(--fw-display);
    letter-spacing: var(--tracking-title);
    font-size: var(--fs-lg);
    line-height: var(--lh-snug);
  }
  .counts {
    font-size: var(--fs-sm);
    color: var(--text-muted);
  }
  .e {
    color: var(--danger);
  }
  .issues {
    display: grid;
    gap: var(--s-1);
  }
  summary {
    display: flex;
    align-items: center;
    gap: var(--s-2);
    cursor: pointer;
    list-style: none;
    padding: var(--s-2) 0;
    font-size: var(--fs-sm);
    color: var(--text);
  }
  summary::-webkit-details-marker {
    display: none;
  }
  summary :global(svg) {
    transition: transform var(--dur-2) var(--ease-out);
  }
  details[open] > summary :global(svg) {
    transform: rotate(90deg);
  }
  .count {
    font-size: var(--fs-xs);
    color: var(--danger);
  }
  [data-severity="warning"] summary,
  [data-severity="warning"] .count {
    color: var(--text-muted);
  }
  ul {
    display: grid;
    gap: var(--s-3);
    padding: 0 0 var(--s-3) var(--s-6);
  }
  li {
    display: grid;
    gap: 2px;
    border-left: 1px solid var(--line);
    padding-left: var(--s-3);
  }
  .lesson {
    font-size: var(--fs-sm);
    color: var(--text);
  }
  [data-severity="warning"] .lesson {
    color: var(--text-muted);
  }
  .msg,
  .hint {
    font-size: var(--fs-xs);
    color: var(--text-muted);
  }
  .where {
    display: flex;
    align-items: center;
    gap: var(--s-2);
    min-width: 0;
  }
  code {
    font-size: var(--fs-xs);
    color: var(--moon);
    user-select: all;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    min-width: 0;
  }
  .copy {
    font-size: var(--fs-xs);
    color: var(--text-muted);
    border: 1px solid var(--line);
    border-radius: var(--r-1);
    padding: 0 var(--s-2);
    flex: none;
  }
  .copy:hover {
    color: var(--text);
  }
  .actions {
    display: flex;
    flex-wrap: wrap;
    gap: var(--s-2);
  }
  .link {
    color: var(--text-muted);
    padding: 0 var(--s-3);
  }
  .link:hover {
    color: var(--text);
  }
</style>
