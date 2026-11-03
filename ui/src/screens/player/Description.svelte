<script lang="ts">
  // Lesson description rendered in a script-less sandboxed frame with a dark
  // theme. Links are intercepted and opened in the system browser.
  import { mediaUrl } from "../../lib/api";
  import { t } from "../../lib/i18n/index.svelte";
  import { buildDescriptionDoc, externalHref } from "../../lib/logic/description";
  import { app } from "../../lib/state/app.svelte";
  import { textSize } from "../../lib/state/textSize.svelte";

  let { path, heading }: { path: string; heading?: string } = $props();
  const uid = $props.id();
  let html = $state<string | null>(null);
  let frame: HTMLIFrameElement | undefined = $state();
  let height = $state(80);

  $effect(() => {
    const p = path;
    html = null;
    app.api
      ?.description(p)
      .then((h) => {
        if (p === path) html = h;
      })
      .catch(() => {
        if (p === path) html = "";
      });
  });

  function themeCss(): string {
    const css = getComputedStyle(document.documentElement);
    const v = (name: string) => css.getPropertyValue(name).trim();
    return `:root{color-scheme:dark}html,body{margin:0;padding:0;background:transparent!important;color:${v("--text-muted")}!important;
font:${textSize.px}px/1.7 ${v("--font-ui")};overflow:hidden;overflow-wrap:anywhere}
body *:not(a){color:inherit!important;background-color:transparent!important;font-family:inherit!important}
h1,h2,h3,h4,strong,b{color:${v("--text")}!important}
h1,h2,h3{font-weight:500;line-height:1.3}
a{color:${v("--moon")}!important;text-underline-offset:3px}
img,video,iframe,table{max-width:100%;height:auto}
hr{border:0;border-top:1px solid ${v("--line")}}
pre{white-space:pre-wrap}`;
  }

  const doc = $derived.by(() => {
    if (html === null) return "";
    const dir = path.split("/").slice(0, -1).join("/");
    const base = app.mediaBase ? `${mediaUrl(app.mediaBase, dir)}/` : "";
    let origin = "";
    try {
      origin = app.mediaBase ? new URL(app.mediaBase).origin : "";
    } catch {
      origin = "";
    }
    return buildDescriptionDoc(html, { base, mediaOrigin: origin, css: themeCss() });
  });

  function fit() {
    try {
      const d = frame?.contentDocument;
      if (d?.body) height = Math.max(40, Math.ceil(d.body.getBoundingClientRect().height) + 8);
    } catch {
      /* cross-origin: keep the default height */
    }
  }

  function onLoad() {
    let d: Document | null | undefined;
    try {
      d = frame?.contentDocument;
    } catch {
      return;
    }
    if (!d) return;
    d.addEventListener("click", (e) => {
      const a = (e.target as Element | null)?.closest?.("a");
      if (!a) return;
      e.preventDefault();
      const url = externalHref(a.getAttribute("href"));
      if (url) void app.api?.openUrl(url);
    });
    d.querySelectorAll("img").forEach((img) => img.addEventListener("load", fit));
    fit();
  }
</script>

<svelte:window onresize={fit} />

{#if html}
  <section class="desc" aria-labelledby="desc-h-{uid}">
    <div class="head">
      <h2 id="desc-h-{uid}" class="eyebrow">{heading ?? t("description")}</h2>
      <div class="size" role="group" aria-label={t("ts_label")}>
        <button type="button" class="icon-btn" onclick={() => textSize.smaller()} disabled={!textSize.canShrink} aria-label={t("ts_smaller")} title={t("ts_smaller")}>
          <span aria-hidden="true">A<small>−</small></span>
        </button>
        <span class="num px" aria-live="polite">{textSize.px}</span>
        <button type="button" class="icon-btn" onclick={() => textSize.bigger()} disabled={!textSize.canGrow} aria-label={t("ts_bigger")} title={t("ts_bigger")}>
          <span aria-hidden="true">A<small>+</small></span>
        </button>
      </div>
    </div>
    <iframe
      bind:this={frame}
      title={heading ?? t("description")}
      sandbox="allow-same-origin"
      srcdoc={doc}
      style:height="{height}px"
      onload={onLoad}
    ></iframe>
  </section>
{/if}

<style>
  .desc {
    display: grid;
    gap: var(--s-3);
  }
  .head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--s-3);
  }
  h2 {
    font-family: var(--font-ui);
  }
  .size {
    display: flex;
    align-items: center;
    gap: var(--s-1);
  }
  .size .icon-btn span {
    font-size: var(--fs-base);
    line-height: 1;
  }
  .size small {
    font-size: 0.7em;
  }
  .px {
    min-width: 2.5ch;
    text-align: center;
    color: var(--text-muted);
    font-size: var(--fs-xs);
  }
  iframe {
    width: 100%;
    border: 0;
    display: block;
    color-scheme: dark;
  }
</style>
