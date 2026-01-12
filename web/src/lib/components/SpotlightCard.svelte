<script lang="ts">
  import { onMount } from 'svelte';

  let div: HTMLDivElement;
  let opacity = 0;
  
  function handleMouseMove(e: MouseEvent) {
    if (!div) return;
    const rect = div.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;
    
    div.style.setProperty("--mouse-x", `${x}px`);
    div.style.setProperty("--mouse-y", `${y}px`);
  }
</script>

<div 
  bind:this={div}
  on:mousemove={handleMouseMove}
  on:mouseenter={() => (opacity = 1)}
  on:mouseleave={() => (opacity = 0)}
  class="relative h-full w-full rounded-xl border border-white/10 bg-white/5 p-8 overflow-hidden group transition-colors hover:border-white/20"
>
  <div 
    class="pointer-events-none absolute -inset-px opacity-0 transition-opacity duration-300 group-hover:opacity-100"
    style="background: radial-gradient(600px circle at var(--mouse-x) var(--mouse-y), rgba(255,255,255,0.06), transparent 40%);"
  ></div>

  <div class="relative z-10">
    <slot />
  </div>
</div>