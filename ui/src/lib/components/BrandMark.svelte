<script lang="ts">
  // The LumenGet badge for small placements: the owner's logo reduced to the
  // plate with the L, the play and the arrow (assets/brand/lumen-mark-small.svg).
  // The platform tiles, the ring and the tray of the full logo turn into noise
  // below ~48px. The full logo is the app icon.
  import mark from "../../assets/brand/lumen-mark-small.svg";

  let { size = 28 }: { size?: number } = $props();
</script>

<img src={mark} width={size} height={size} alt="" aria-hidden="true" draggable="false" />

<style>
  img {
    display: block;
    flex: none;
  }
</style>
