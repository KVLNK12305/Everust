<script lang="ts">
  import { onMount } from 'svelte';
  import { gsap } from 'gsap';
  import { TextPlugin } from 'gsap/TextPlugin';
  import SpotlightCard from '$lib/components/SpotlightCard.svelte';
  import { Terminal, Cpu, Activity, ShieldCheck } from 'lucide-svelte';

  gsap.registerPlugin(TextPlugin);

  onMount(() => {
    const tl = gsap.timeline();

    // 1. Reveal Title
    tl.from(".hero-title", {
      y: 50,
      opacity: 0,
      duration: 1,
      ease: "power4.out"
    })
    // 2. Scramble Text Effect (The "Hacker" effect)
    .to(".scramble-text", {
      duration: 1.5,
      text: {
        value: "SYSTEMS • SECURITY • RUST",
        delimiter: ""
      },
      ease: "none"
    }, "-=0.5")
    // 3. Reveal Cards
    .from(".feature-card", {
      y: 30,
      opacity: 0,
      stagger: 0.1,
      duration: 0.8,
      ease: "back.out(1.7)"
    }, "-=0.5");
  });
</script>

<div class="container mx-auto px-6 py-24">
  <div class="max-w-4xl mx-auto text-center mb-32 relative">
    <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[500px] h-[300px] bg-rust/20 blur-[100px] rounded-full pointer-events-none"></div>

    <h1 class="hero-title text-7xl md:text-9xl font-bold tracking-tighter mb-6 bg-clip-text text-transparent bg-gradient-to-b from-white to-white/40">
      EVERUST
    </h1>
    
    <div class="h-8 mb-8">
      <p class="scramble-text font-mono text-rust text-lg tracking-widest uppercase">
        INITIALIZING...
      </p>
    </div>

    <p class="hero-title text-xl text-slate-400 max-w-2xl mx-auto leading-relaxed">
      Not just a tutorial. A living, breathing system. 
      Documenting the journey from <span class="text-white">Syntax</span> to <span class="text-white">Silicon</span>.
    </p>

    <div class="mt-10 flex gap-4 justify-center hero-title opacity-0">
      <a href="/foundations" class="px-8 py-3 bg-white text-black font-semibold rounded-full hover:scale-105 transition-transform">
        Start Core
      </a>
      <a href="/experiments" class="px-8 py-3 bg-white/5 border border-white/10 text-white rounded-full hover:bg-white/10 transition-colors">
        View Labs
      </a>
    </div>
  </div>

  <div class="grid grid-cols-1 md:grid-cols-3 gap-6 max-w-6xl mx-auto">
    
    <a href="/foundations" class="feature-card block group">
      <SpotlightCard>
        <div class="h-12 w-12 bg-blue-500/10 rounded-lg flex items-center justify-center mb-4 text-blue-400">
          <Terminal size={24} />
        </div>
        <h3 class="text-xl font-bold mb-2 group-hover:text-blue-400 transition-colors">Foundations</h3>
        <p class="text-sm text-slate-400 leading-relaxed">
          Pure Rust. No Frameworks. Understanding ownership, lifetimes, and the borrow checker at the deepest level.
        </p>
      </SpotlightCard>
    </a>

    <a href="/experiments" class="feature-card block group">
      <SpotlightCard>
        <div class="h-12 w-12 bg-rust/10 rounded-lg flex items-center justify-center mb-4 text-rust">
          <Cpu size={24} />
        </div>
        <h3 class="text-xl font-bold mb-2 group-hover:text-rust transition-colors">Experiments</h3>
        <p class="text-sm text-slate-400 leading-relaxed">
          Unsafe Rust, concurrency models, and embedded systems logic. Where the theory hits the metal.
        </p>
      </SpotlightCard>
    </a>

    <a href="/benches" class="feature-card block group">
      <SpotlightCard>
        <div class="h-12 w-12 bg-emerald-500/10 rounded-lg flex items-center justify-center mb-4 text-emerald-400">
          <Activity size={24} />
        </div>
        <h3 class="text-xl font-bold mb-2 group-hover:text-emerald-400 transition-colors">Benches</h3>
        <p class="text-sm text-slate-400 leading-relaxed">
          Data-driven decisions. Visualizing performance trade-offs between different Rust approaches.
        </p>
      </SpotlightCard>
    </a>

  </div>
</div>