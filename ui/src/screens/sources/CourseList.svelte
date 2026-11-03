<script lang="ts">
  import { countStatuses, filterAndOrder, type CourseFilter } from "../../lib/logic/courseFilter";
  import { t } from "../../lib/i18n/index.svelte";
  import { localStatusByFolder } from "../../lib/logic/localStatus";
  import { app } from "../../lib/state/app.svelte";
  import type { DownloadItemSession } from "../../lib/state/downloads.svelte";
  import type { PlatformId } from "../../lib/types";
  import ItemRow from "./ItemRow.svelte";

  export interface CourseListItem {
    key: string;
    title: string;
    coverUrl: string | null;
    facts: string[];
    folder: string;
    issue?: string | null;
  }

  /** Directory of each platform under the library root. */
  const FOLDER_ROOT = { hotmart: "Hotmart", pluralsight: "Pluralsight", goldenlife: "GoldenLife", harbiz: "Harbiz" } as const;

  let { session, platform, items, heading, emptyText }: {
    session: DownloadItemSession;
    platform: "hotmart" | "pluralsight" | "goldenlife" | "harbiz";
    items: CourseListItem[];
    heading: string;
    emptyText: string;
  } = $props();

  let filter = $state<CourseFilter>("all");
  const rows = $derived(items.map((course) => ({ course, status: localStatusByFolder(app.library, platform, course.folder) })));
  const counts = $derived(countStatuses(rows.map((r) => r.status)));
  const shown = $derived(filterAndOrder(rows, filter));
  const FILTERS: { id: CourseFilter; label: "cf_all" | "cf_incomplete" | "cf_none" | "cf_complete" }[] = [
    { id: "all", label: "cf_all" },
    { id: "incomplete", label: "cf_incomplete" },
    { id: "none", label: "cf_none" },
    { id: "complete", label: "cf_complete" },
  ];
</script>

<section aria-label={heading}>
  <h2 class="eyebrow section">{heading}</h2>
  {#if items.length}
    {#if items.length > 1}
      <div class="filters" role="group" aria-label={t("cf_label")}>
        {#each FILTERS as f (f.id)}
          <button type="button" class="chip" aria-pressed={filter === f.id} onclick={() => (filter = f.id)}>
            {t(f.label)} <span class="num">{counts[f.id]}</span>
          </button>
        {/each}
      </div>
      {#if filter === "incomplete" && counts.incomplete > 0}<p class="hint">{t("cf_incomplete_hint")}</p>{/if}
    {/if}
    <ul class="items">
      {#each shown as { course, status } (course.key)}
        <ItemRow
          {session}
          {platform}
          itemKey={course.key}
          name={course.title}
          coverUrl={course.coverUrl}
          facts={course.facts}
          local={status}
          folder={`${FOLDER_ROOT[platform]}/${course.folder}`}
          disabledReason={course.issue}
        />
      {/each}
    </ul>
    {#if !shown.length}<p class="muted">{t("cf_empty")}</p>{/if}
  {:else}
    <p class="muted">{emptyText}</p>
  {/if}
</section>

<style>
  .section {
    font-family: var(--font-ui);
    margin-bottom: var(--s-2);
  }
  .items {
    border-top: 1px solid var(--line);
  }
  .muted {
    color: var(--text-muted);
  }
  .filters {
    display: flex;
    flex-wrap: wrap;
    gap: var(--s-2);
    padding: var(--s-3) 0 var(--s-4);
  }
  .chip {
    display: inline-flex;
    align-items: center;
    gap: var(--s-2);
    min-height: 36px;
    padding: 0 var(--s-4);
    border: 1px solid var(--line);
    border-radius: var(--r-pill, 999px);
    background: transparent;
    color: var(--text-muted);
    font-size: var(--fs-sm);
    cursor: pointer;
  }
  .chip[aria-pressed="true"] {
    color: var(--text);
    border-color: var(--lumen);
  }
  .chip .num {
    color: var(--text-faint);
  }
  .chip[aria-pressed="true"] .num {
    color: var(--lumen);
  }
  .hint {
    color: var(--text-muted);
    font-size: var(--fs-xs);
    padding-bottom: var(--s-3);
  }
  @media (pointer: coarse) {
    .chip {
      min-height: 44px;
    }
  }
</style>
