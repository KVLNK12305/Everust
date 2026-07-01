<script lang="ts">
  import { onMount } from "svelte";
  import { gsap } from "gsap";
  import { BarChart3, Timer, Database, Zap, AlertCircle, Terminal, Activity } from "lucide-svelte";

  // Mock data for the "Offline Sensors"
  const sensors = [
    {
      label: "Throughput",
      desc: "Ops/sec & bandwidth saturation",
      icon: Zap,
      color: "text-amber-400",
      border: "group-hover:border-amber-500/50",
      bg: "group-hover:bg-amber-500/10",
      slug: "throughput"
    },
    {
      label: "Latency",
      desc: "p99 / p99.9 jitter analysis",
      icon: Timer,
      color: "text-cyan-400",
      border: "group-hover:border-cyan-500/50",
      bg: "group-hover:bg-cyan-500/10",
      slug: "latency"
    },
    {
      label: "Memory",
      desc: "Allocator pressure & cache misses",
      icon: Database,
      color: "text-rose-400",
      border: "group-hover:border-rose-500/50",
      bg: "group-hover:bg-rose-500/10",
      slug: "memory"
    }
  ];

  onMount(() => {
    const tl = gsap.timeline();

    // 1. Hero Reveal
    tl.to(".hero-fade", {
      opacity: 1,
      y: 0,
      stagger: 0.1,
      duration: 0.8,
      ease: "power3.out"
    })
    // 2. Card Reveal
    .to(".sensor-card", {
      opacity: 1,
      y: 0,
      stagger: 0.15,
      duration: 0.6,
      ease: "back.out(1.2)"
    }, "-=0.4");

    // 3. Scanline Animation (CSS is handled, but we can boost it via GSAP if needed)
    gsap.to(".signal-line", {
      x: "100%",
      duration: 2,
      repeat: -1,
      ease: "linear"
    });
  });
</script>

<div class="min-h-screen bg-[#0a0a0a] text-slate-200 px-6 py-24 font-sans selection:bg-emerald-500 selection:text-white overflow-x-hidden">

  <div class="fixed inset-0 pointer-events-none z-0">
    <div class="absolute top-1/3 left-1/2 -translate-x-1/2 w-[800px] h-[400px] bg-emerald-500/5 rounded-full blur-[120px]"></div>
    <div class="absolute inset-0 bg-[linear-gradient(rgba(255,255,255,0.02)_1px,transparent_1px),linear-gradient(90deg,rgba(255,255,255,0.02)_1px,transparent_1px)] bg-[size:4rem_4rem] [mask-image:radial-gradient(ellipse_60%_50%_at_50%_0%,#000_70%,transparent_100%)]"></div>
  </div>

  <div class="relative z-10 max-w-5xl mx-auto">

    <header class="text-center mb-20">
      <div class="hero-fade opacity-0 translate-y-4 mb-6 inline-flex justify-center">
        <div class="flex items-center gap-2 px-3 py-1 rounded-full border border-emerald-500/30 bg-emerald-500/10 text-emerald-400 text-xs font-mono tracking-widest uppercase">
          <Activity size={12} class="animate-pulse" />
          Measurement Pipeline
        </div>
      </div>

      <h1 class="hero-fade opacity-0 translate-y-4 text-5xl md:text-7xl font-bold tracking-tighter text-white mb-6">
        Benchmarks
      </h1>

      <p class="hero-fade opacity-0 translate-y-4 text-xl text-slate-400 leading-relaxed max-w-2xl mx-auto">
        Performance is not an opinion. It is an <span class="text-emerald-400">observable property</span>.
        <br>
        <span class="text-sm opacity-50 mt-2 block font-mono">Current Status: Awaiting Test Vectors</span>
      </p>
    </header>

    <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-20">
      {#each sensors as sensor}
        <a href={`/benches/${sensor.slug}`} class="sensor-card opacity-0 translate-y-8 group relative bg-white/5 border border-white/10 rounded-xl p-6 overflow-hidden transition-all duration-300 hover:-translate-y-1 hover:bg-white/[0.07] block no-underline text-inherit {sensor.border} {sensor.bg}">
          
          <div class="absolute inset-0 opacity-0 group-hover:opacity-100 transition-opacity duration-500 pointer-events-none">
            <div class="signal-line absolute top-0 left-0 w-full h-full bg-gradient-to-r from-transparent via-white/5 to-transparent -translate-x-full"></div>
          </div>

          <div class="flex items-center gap-4 mb-4">
            <div class="p-3 bg-black/40 rounded-lg {sensor.color} border border-white/5">
              <svelte:component this={sensor.icon} size={24} />
            </div>
            <div class="flex flex-col">
              <span class="text-lg font-bold text-white">{sensor.label}</span>
              <span class="text-[10px] font-mono text-slate-500 uppercase tracking-wider">No Signal</span>
            </div>
          </div>
          
          <p class="text-sm text-slate-400 leading-relaxed mb-6">
            {sensor.desc}
          </p>

          <div class="h-16 flex items-end gap-1 opacity-30 group-hover:opacity-60 transition-opacity">
            <div class="w-1/5 bg-current h-[20%] rounded-t-sm animate-pulse {sensor.color}"></div>
            <div class="w-1/5 bg-current h-[40%] rounded-t-sm animate-pulse delay-75 {sensor.color}"></div>
            <div class="w-1/5 bg-current h-[60%] rounded-t-sm animate-pulse delay-100 {sensor.color}"></div>
            <div class="w-1/5 bg-current h-[30%] rounded-t-sm animate-pulse delay-150 {sensor.color}"></div>
            <div class="w-1/5 bg-current h-[10%] rounded-t-sm animate-pulse delay-200 {sensor.color}"></div>
          </div>

        </a>
      {/each}
    </div>

    <div class="hero-fade opacity-0 translate-y-4 border border-emerald-500/20 bg-emerald-500/5 rounded-lg p-6 flex flex-col md:flex-row items-center gap-6 relative overflow-hidden">
      <div class="absolute inset-0 opacity-[0.03] bg-[linear-gradient(45deg,#000_25%,transparent_25%,transparent_50%,#000_50%,#000_75%,transparent_75%,transparent)] bg-[length:20px_20px]"></div>

      <div class="p-4 bg-emerald-500/10 rounded-full text-emerald-400 shrink-0 relative z-10">
        <BarChart3 size={32} />
      </div>
      
      <div class="text-center md:text-left relative z-10 flex-1">
        <h3 class="text-emerald-400 font-mono text-sm font-bold uppercase tracking-widest mb-2">
          Criterion.rs Missing
        </h3>
        <p class="text-slate-400 text-sm leading-relaxed max-w-xl">
          Benchmarks must be statistically significant. I am currently writing the harness to minimize OS noise and ensure reproducible visuals.
        </p>
      </div>

      <div class="font-mono text-xs text-emerald-500/80 bg-black/40 px-4 py-3 rounded border border-emerald-500/20 whitespace-nowrap shrink-0">
        <span class="text-slate-500">$</span> cargo bench --no-run
        <br>
        <span class="text-amber-500">>></span> status: <span class="text-white font-bold">CALIBRATING...</span>
      </div>
    </div>

  </div>
</div>