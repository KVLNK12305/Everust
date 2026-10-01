<script lang="ts">
  import { onMount } from 'svelte';
  import { gsap } from 'gsap';
  // ✅ Correct import for production to prevent tree-shaking issues
  import TextPlugin from 'gsap/dist/TextPlugin';
  import SpotlightCard from '$lib/components/SpotlightCard.svelte';
  import { Terminal, Cpu, Activity } from 'lucide-svelte';

  onMount(() => {
    // Register inside onMount (safe for SSR)
    gsap.registerPlugin(TextPlugin);

    const tl = gsap.timeline();

    // ✅ SAFE REVEAL STRATEGY: 
    // 1. Force hidden state instantly with JS (prevents FOUC)
    // 2. Animate TO visible. This guarantees opacity ends at 1.
    tl.set(".hero-anim", { 
      y: 50, 
      opacity: 0 
    })
    
    // 1. Reveal Title & Buttons
    .to(".hero-anim", {
      y: 0,
      opacity: 1,
      duration: 1,
      stagger: 0.1, // Adds a nice delay between Title -> Buttons
      ease: "power4.out"
    })
    
    // 2. Scramble Text Effect
    .to(".scramble-text", {
      duration: 1.5,
      text: {
        value: "SYSTEMS FLIGHT RECORDER & WORKBENCH",
        delimiter: ""
      },
      ease: "none"
    }, "-=0.8") // Overlap slightly for speed
    
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

<svelte:head>
  <title>Everust — Visual Systems Flight Recorder for Rust</title>
</svelte:head>

<div class="container mx-auto px-6 py-24">
  <div class="max-w-4xl mx-auto text-center mb-28 relative">
    
    <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[550px] h-[320px] bg-rust/20 blur-[120px] rounded-full pointer-events-none"></div>

    <h1 class="hero-anim text-7xl md:text-9xl font-bold tracking-tighter mb-4 bg-clip-text text-transparent bg-gradient-to-b from-white to-white/40">
      EVERUST
    </h1>
    
    <div class="h-8 mb-6">
      <p class="scramble-text font-mono text-rust text-sm md:text-base tracking-[0.25em] uppercase font-semibold">
        INITIALIZING RECORDER...
      </p>
    </div>

    <p class="hero-anim text-lg md:text-xl text-slate-300 max-w-2xl mx-auto leading-relaxed font-light">
      Continuous in-process flight recording and post-mortem visual investigation for Rust.
      Deterministically diagnose <span class="text-white font-medium">async runtime stalls</span>, <span class="text-white font-medium">lock contention</span>, and <span class="text-white font-medium">task starvation</span>.
    </p>

    <div class="mt-10 flex flex-wrap gap-4 justify-center hero-anim">
      <a href="/workbench" class="px-8 py-3.5 bg-rust hover:bg-rust/90 text-white font-semibold rounded-full shadow-[0_0_25px_rgba(206,75,41,0.4)] hover:scale-105 transition-all flex items-center gap-2">
        <Activity size={18} />
        <span>Launch Workbench</span>
      </a>
      <a href="/experiments" class="px-8 py-3.5 bg-white/5 border border-white/10 text-white rounded-full hover:bg-white/10 transition-colors flex items-center gap-2">
        <Cpu size={18} />
        <span>Workload Models</span>
      </a>
      <a href="/foundations" class="px-6 py-3.5 text-slate-400 hover:text-white transition-colors text-sm font-mono flex items-center gap-1.5">
        <Terminal size={16} />
        <span>Foundations</span>
      </a>
    </div>
  </div>

  <div class="grid grid-cols-1 md:grid-cols-3 gap-6 max-w-6xl mx-auto">
    
    <a href="/workbench" class="feature-card block group">
      <SpotlightCard>
        <div class="h-12 w-12 bg-rust/10 border border-rust/20 rounded-lg flex items-center justify-center mb-4 text-rust group-hover:scale-110 transition-transform">
          <Activity size={24} />
        </div>
        <h3 class="text-xl font-bold mb-2 group-hover:text-rust transition-colors text-white">Investigation Workbench</h3>
        <p class="text-sm text-slate-400 leading-relaxed">
          Interactive execution swimlanes, scrub cursor, event stream inspection, and automated deterministic incident diagnoses.
        </p>
      </SpotlightCard>
    </a>

    <a href="/experiments" class="feature-card block group">
      <SpotlightCard>
        <div class="h-12 w-12 bg-emerald-500/10 border border-emerald-500/20 rounded-lg flex items-center justify-center mb-4 text-emerald-400 group-hover:scale-110 transition-transform">
          <Cpu size={24} />
        </div>
        <h3 class="text-xl font-bold mb-2 group-hover:text-emerald-400 transition-colors text-white">Workload Models</h3>
        <p class="text-sm text-slate-400 leading-relaxed">
          Reproducible lock contention, async starvation, and zero-copy pipelines designed to generate real flight recordings.
        </p>
      </SpotlightCard>
    </a>

    <a href="/foundations" class="feature-card block group">
      <SpotlightCard>
        <div class="h-12 w-12 bg-blue-500/10 border border-blue-500/20 rounded-lg flex items-center justify-center mb-4 text-blue-400 group-hover:scale-110 transition-transform">
          <Terminal size={24} />
        </div>
        <h3 class="text-xl font-bold mb-2 group-hover:text-blue-400 transition-colors text-white">Foundations Reference</h3>
        <p class="text-sm text-slate-400 leading-relaxed">
          The engineering bedrock. Move semantics, borrowing rules matrix, and zero-allocation slice abstractions.
        </p>
      </SpotlightCard>
    </a>

  </div>
</div>