<script lang="ts">
  import { onMount, onDestroy } from "svelte";

  export let label: string = "";
  export let fromFontVariationSettings: string = "'wght' 400, 'opsz' 9";
  export let toFontVariationSettings: string = "'wght' 900, 'opsz' 40";
  export let containerRef: HTMLElement | null = null;
  export let radius: number = 100;
  export let falloff: "linear" | "exponential" | "gaussian" = "linear";
  export let className: string = "";

  let spanEl: HTMLElement;
  let letterEls: HTMLElement[] = [];
  let mouseX = 0;
  let mouseY = 0;
  let lastX: number | null = null;
  let lastY: number | null = null;
  let frameId: number;

  // Parse font variation settings
  type ParsedSetting = { axis: string; fromValue: number; toValue: number };

  function parseSettings(str: string): Map<string, number> {
    const map = new Map<string, number>();
    str.split(",").map(s => s.trim()).forEach(s => {
      const parts = s.split(" ");
      const name = parts[0].replace(/['"]/g, "");
      const value = parseFloat(parts[1]);
      map.set(name, value);
    });
    return map;
  }

  let parsedSettings: ParsedSetting[] = [];
  $: {
    const fromMap = parseSettings(fromFontVariationSettings);
    const toMap = parseSettings(toFontVariationSettings);
    parsedSettings = Array.from(fromMap.entries()).map(([axis, fromValue]) => ({
      axis,
      fromValue,
      toValue: toMap.get(axis) ?? fromValue
    }));
  }

  function calculateDistance(x1: number, y1: number, x2: number, y2: number): number {
    return Math.sqrt((x2 - x1) ** 2 + (y2 - y1) ** 2);
  }

  function calculateFalloff(distance: number): number {
    const norm = Math.min(Math.max(1 - distance / radius, 0), 1);
    switch (falloff) {
      case "exponential": return norm ** 2;
      case "gaussian": return Math.exp(-((distance / (radius / 2)) ** 2) / 2);
      case "linear":
      default: return norm;
    }
  }

  function handleMouseMove(ev: MouseEvent) {
    if (containerRef) {
      const rect = containerRef.getBoundingClientRect();
      mouseX = ev.clientX - rect.left;
      mouseY = ev.clientY - rect.top;
    } else {
      mouseX = ev.clientX;
      mouseY = ev.clientY;
    }
  }

  function handleTouchMove(ev: TouchEvent) {
    const touch = ev.touches[0];
    if (containerRef) {
      const rect = containerRef.getBoundingClientRect();
      mouseX = touch.clientX - rect.left;
      mouseY = touch.clientY - rect.top;
    } else {
      mouseX = touch.clientX;
      mouseY = touch.clientY;
    }
  }

  function loop() {
    if (!containerRef) { frameId = requestAnimationFrame(loop); return; }
    if (lastX === mouseX && lastY === mouseY) { frameId = requestAnimationFrame(loop); return; }
    lastX = mouseX;
    lastY = mouseY;

    const containerRect = containerRef.getBoundingClientRect();

    for (let i = 0; i < letterEls.length; i++) {
      const el = letterEls[i];
      if (!el) continue;

      const rect = el.getBoundingClientRect();
      const cx = rect.left + rect.width / 2 - containerRect.left;
      const cy = rect.top + rect.height / 2 - containerRect.top;
      const distance = calculateDistance(mouseX, mouseY, cx, cy);

      if (distance >= radius) {
        el.style.fontVariationSettings = fromFontVariationSettings;
      } else {
        const fv = calculateFalloff(distance);
        const newSettings = parsedSettings
          .map(({ axis, fromValue, toValue }) => `'${axis}' ${fromValue + (toValue - fromValue) * fv}`)
          .join(", ");
        el.style.fontVariationSettings = newSettings;
      }
    }

    frameId = requestAnimationFrame(loop);
  }

  onMount(() => {
    window.addEventListener("mousemove", handleMouseMove);
    window.addEventListener("touchmove", handleTouchMove);
    frameId = requestAnimationFrame(loop);
  });

  onDestroy(() => {
    window.removeEventListener("mousemove", handleMouseMove);
    window.removeEventListener("touchmove", handleTouchMove);
    cancelAnimationFrame(frameId);
  });

  // Split label into words and letters
  $: words = label.split(" ");
</script>

<span bind:this={spanEl} class="inline {className}" style="font-family: 'Roboto Flex', sans-serif;">
  {#each words as word, wordIndex}
    <span class="inline-block whitespace-nowrap">
      {#each word.split("") as letter, li}
        {@const idx = words.slice(0, wordIndex).reduce((sum, w) => sum + w.length, 0) + li}
        <span
          bind:this={letterEls[idx]}
          class="inline-block"
          style="font-variation-settings: {fromFontVariationSettings};"
          aria-hidden="true"
        >{letter}</span>
      {/each}
    </span>{#if wordIndex < words.length - 1}<span class="inline-block">&nbsp;</span>{/if}
  {/each}
  <span class="sr-only">{label}</span>
</span>
