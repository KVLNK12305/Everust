<script lang="ts">
  import '../app.css';
  import { onMount } from 'svelte';
  import { gsap } from 'gsap';
  import Navbar from '$lib/components/Navbar.svelte'; // Assuming you have this
  import { page } from '$app/stores';

  let transitionNode: HTMLElement;

  // Simple Page Transition
  $: if (mounted && transitionNode && $page.url.pathname) {
  gsap.fromTo(
    transitionNode,
    { opacity: 0, y: 10 },
    { opacity: 1, y: 0, duration: 0.5, ease: "power2.out" }
  );
}

  let mounted = false;

  onMount(() => {
    // Subtle background grain animation
    mounted = true;
    gsap.to(".noise-bg", {
      backgroundPosition: "100% 100%",
      duration: 2,
      repeat: -1,
      ease: "steps(10)"
    });
  });
</script>

<div class="min-h-screen bg-[#0a0a0a] text-slate-200 selection:bg-rust selection:text-white overflow-x-hidden font-sans">
  
  <div class="noise-bg fixed inset-0 opacity-[0.03] pointer-events-none z-50 mix-blend-overlay"></div>
  
  <Navbar />

  <main bind:this={transitionNode} class="relative z-10 min-h-[calc(100vh-80px)]">
    <slot />
  </main>

  <footer class="fixed bottom-0 left-0 w-full border-t border-white/5 bg-[#0a0a0a]/80 backdrop-blur text-[10px] font-mono text-slate-500 py-1 px-4 flex justify-between items-center z-40">
    <div class="flex gap-4">
      <span class="flex items-center gap-2">
        <span class="w-1.5 h-1.5 rounded-full bg-emerald-500 animate-pulse"></span>
        SYSTEM: ONLINE
      </span>
      <span>MEM: SAFE</span>
    </div>
    <div class="uppercase tracking-widest opacity-50">Everust v0.1.0</div>
  </footer>
</div>

<style>
  .noise-bg {
    background-image: url("data:image/svg+xml,%3Csvg viewBox='0 0 200 200' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='noiseFilter'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.65' numOctaves='3' stitchTiles='stitch'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23noiseFilter)'/%3E%3C/svg%3E");
  }
</style>