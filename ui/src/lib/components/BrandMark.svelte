<script lang="ts">
  // The LumenGet logo (assets/lumen-logo.svg), the same artwork as the app icon.
  import mark from "../../assets/brand/lumen-logo.svg";

  let { size = 28 }: { size?: number } = $props();
</script>

<img src={mark} width={size} height={size} alt="" aria-hidden="true" draggable="false" />

<style>
  img {
    display: block;
    flex: none;
  }
</style>
