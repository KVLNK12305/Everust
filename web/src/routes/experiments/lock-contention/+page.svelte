<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { gsap } from "gsap";
  import { ArrowLeft, Play, Pause, RotateCcw, AlertTriangle, Shield, Lock, Unlock, Users, Gauge, Code2, Share2, Eye, GraduationCap, BarChart3, TrendingUp, Lightbulb } from "lucide-svelte";
  import VariableProximity from "$lib/components/VariableProximity.svelte";

  let pageContainer: HTMLElement;

  // ─── Simulation State ───
  let mode: "unfair" | "fair" = "unfair";
  let arcView: "simple" | "technical" = "simple";
  let threadCount = 4;
  let speed = 1;
  let running = false;
  let intervalId: ReturnType<typeof setInterval> | null = null;
  let sharedCounter = 0;
  let tick = 0;

  type ThreadState = "IDLE" | "ACQUIRING" | "WRITING" | "BLOCKED" | "STARVING";
  
  interface ThreadData {
    id: number;
    writes: number;
    state: ThreadState;
    blockedTicks: number;
    name: string;
  }

  let threads: ThreadData[] = [];
  let fairQueueIdx = 0;

  // History for the live graph (last N ticks of each thread's cumulative writes)
  const HISTORY_LEN = 40;
  let writeHistory: number[][] = []; // writeHistory[threadIdx][tickIdx]

  const threadNames = ["Atlas", "Bravo", "Cipher", "Delta", "Echo", "Foxtrot", "Ghost", "Helix"];
  const threadColors = ["#34d399", "#f59e0b", "#f87171", "#60a5fa", "#a78bfa", "#fb923c", "#2dd4bf", "#e879f9"];

  function initThreads() {
    threads = Array.from({ length: threadCount }, (_, i) => ({
      id: i,
      writes: 0,
      state: "IDLE",
      blockedTicks: 0,
      name: threadNames[i]
    }));
    sharedCounter = 0;
    tick = 0;
    fairQueueIdx = 0;
    writeHistory = Array.from({ length: threadCount }, () => [0]);
  }

  function pushHistory() {
    for (let i = 0; i < threads.length; i++) {
      if (!writeHistory[i]) writeHistory[i] = [];
      writeHistory[i].push(threads[i].writes);
      if (writeHistory[i].length > HISTORY_LEN) writeHistory[i].shift();
    }
    writeHistory = writeHistory; // trigger reactivity
  }

  // ─── Unfair Mode ───
  function simulateUnfair() {
    tick++;
    let winnerIdx: number;
    if (Math.random() < 0.72) {
      winnerIdx = 0;
    } else {
      winnerIdx = 1 + Math.floor(Math.random() * (threads.length - 1));
    }
    threads = threads.map((t, i) => {
      if (i === winnerIdx) {
        return { ...t, writes: t.writes + 1, state: "WRITING" as ThreadState, blockedTicks: 0 };
      } else {
        const nb = t.blockedTicks + 1;
        return { ...t, state: (nb > 8 ? "STARVING" : "BLOCKED") as ThreadState, blockedTicks: nb };
      }
    });
    sharedCounter++;
    pushHistory();
  }

  // ─── Fair Mode (Rust FIFO) ───
  function simulateFair() {
    tick++;
    const winnerIdx = fairQueueIdx % threads.length;
    fairQueueIdx++;
    threads = threads.map((t, i) => {
      if (i === winnerIdx) {
        return { ...t, writes: t.writes + 1, state: "WRITING" as ThreadState, blockedTicks: 0 };
      } else {
        return { ...t, state: "ACQUIRING" as ThreadState, blockedTicks: 0 };
      }
    });
    sharedCounter++;
    pushHistory();
  }

  function startSim() {
    if (running) return;
    running = true;
    intervalId = setInterval(() => {
      if (mode === "unfair") simulateUnfair();
      else simulateFair();
    }, Math.max(50, 300 / speed));
  }

  function stopSim() {
    running = false;
    if (intervalId) clearInterval(intervalId);
    intervalId = null;
  }

  function resetSim() {
    stopSim();
    initThreads();
  }

  function toggleMode(newMode: "unfair" | "fair") {
    mode = newMode;
    resetSim();
  }

  // ─── Derived ───
  $: maxWrites = Math.max(1, ...threads.map(t => t.writes));
  $: minWrites = Math.min(...threads.map(t => t.writes));
  $: starvationIndex = maxWrites > 0 ? (maxWrites / Math.max(1, minWrites)).toFixed(1) : "1.0";
  $: totalWrites = threads.reduce((sum, t) => sum + t.writes, 0);

  // Per-thread write share for the pie/donut
  $: writeShares = threads.map(t => totalWrites > 0 ? ((t.writes / totalWrites) * 100).toFixed(1) : "0.0");

  // Restart interval when speed changes
  $: if (running && speed) {
    stopSim();
    running = false;
    startSim();
  }

  function getStateColor(state: ThreadState): string {
    switch (state) {
      case "WRITING": return "bg-emerald-500";
      case "ACQUIRING": return "bg-amber-500";
      case "BLOCKED": return "bg-red-500/60";
      case "STARVING": return "bg-red-600";
      default: return "bg-slate-600";
    }
  }

  function getStateDot(state: ThreadState): string {
    switch (state) {
      case "WRITING": return "bg-emerald-400";
      case "ACQUIRING": return "bg-amber-400";
      case "BLOCKED": return "bg-red-400";
      case "STARVING": return "bg-red-500";
      default: return "bg-slate-500";
    }
  }

  function getStateTextColor(state: ThreadState): string {
    switch (state) {
      case "WRITING": return "text-emerald-400";
      case "ACQUIRING": return "text-amber-400";
      case "BLOCKED": return "text-red-400";
      case "STARVING": return "text-red-500";
      default: return "text-slate-500";
    }
  }

  // ─── SVG line chart helpers ───
  function getGraphPath(threadIdx: number): string {
    const data = writeHistory[threadIdx] || [];
    if (data.length < 2) return "";
    const graphMax = Math.max(1, ...writeHistory.flat());
    const w = 100; // viewBox width percentage
    const h = 100; // viewBox height
    const step = w / (HISTORY_LEN - 1);
    return data.map((v, i) => {
      const x = i * step;
      const y = h - (v / graphMax) * (h - 10);
      return `${i === 0 ? 'M' : 'L'}${x.toFixed(1)},${y.toFixed(1)}`;
    }).join(' ');
  }

  // Arc ref-count diagram positions (spread out in a 200x120 viewBox)
  const arcNodes = [
    { label: "Main", x: 100, y: 10 },
    { label: "Thread 1", x: 30, y: 105 },
    { label: "Thread 2", x: 100, y: 110 },
    { label: "Thread 3", x: 170, y: 105 },
  ];

  // ─── Lifecycle ───
  onMount(() => {
    initThreads();
    const tl = gsap.timeline();
    tl.to(".hero-anim", { y: 0, opacity: 1, stagger: 0.08, duration: 0.7, ease: "power3.out" })
      .to(".section-anim", { y: 0, opacity: 1, stagger: 0.1, duration: 0.6, ease: "power2.out" }, "-=0.3");
  });

  onDestroy(() => { stopSim(); });
</script>

<div class="min-h-screen bg-[#0a0a0a] text-slate-200 px-4 md:px-6 py-20 font-sans selection:bg-emerald-500 selection:text-white overflow-x-hidden">

  <!-- Background -->
  <div class="fixed inset-0 pointer-events-none z-0">
    <div class="absolute top-0 left-1/3 w-[600px] h-[600px] bg-emerald-500/5 rounded-full blur-[150px]"></div>
    <div class="absolute bottom-1/4 right-1/4 w-[400px] h-[400px] bg-red-500/5 rounded-full blur-[120px]"></div>
    <div class="absolute inset-0 bg-[linear-gradient(rgba(255,255,255,0.02)_1px,transparent_1px),linear-gradient(90deg,rgba(255,255,255,0.02)_1px,transparent_1px)] bg-[size:4rem_4rem] [mask-image:radial-gradient(ellipse_60%_50%_at_50%_0%,#000_70%,transparent_100%)]"></div>
  </div>

  <div class="relative z-10 max-w-6xl mx-auto" bind:this={pageContainer}>

    <!-- Back -->
    <a href="/experiments" class="hero-anim opacity-0 translate-y-4 inline-flex items-center gap-2 text-slate-500 hover:text-emerald-400 transition-colors mb-12 group font-mono text-sm">
      <ArrowLeft size={16} class="group-hover:-translate-x-1 transition-transform" />
      experiments/
    </a>

    <!-- ═══════════════════════════════════ -->
    <!-- HERO                               -->
    <!-- ═══════════════════════════════════ -->
    <header class="mb-16">
      <div class="hero-anim opacity-0 translate-y-4 mb-4">
        <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full border border-emerald-500/30 bg-emerald-500/10 text-emerald-400 text-xs font-mono tracking-widest uppercase">
          <span class="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse"></span>
          EXP-01 &middot; ACTIVE
        </div>
      </div>
      <h1 class="hero-anim opacity-0 translate-y-4 text-4xl md:text-6xl font-bold tracking-tighter text-white mb-4">
        Lock Contention &
        <span class="text-transparent bg-clip-text bg-gradient-to-r from-emerald-400 to-teal-300">Mutex Starvation</span>
      </h1>
      <p class="hero-anim opacity-0 translate-y-4 text-xl text-slate-400 leading-relaxed max-w-3xl">
        <VariableProximity
          label="When multiple threads fight for the same lock, some can be perpetually denied access. This is starvation — and it's one of concurrency's most insidious bugs."
          className="variable-proximity-hero"
          fromFontVariationSettings="'wght' 400, 'opsz' 9"
          toFontVariationSettings="'wght' 900, 'opsz' 40"
          containerRef={pageContainer}
          radius={80}
          falloff="linear"
        />
      </p>
    </header>

    <!-- ═══════════════════════════════════ -->
    <!-- VISUAL THEORY — 3 cards with       -->
    <!-- mini diagrams instead of text walls -->
    <!-- ═══════════════════════════════════ -->
    <div class="section-anim opacity-0 translate-y-6 grid grid-cols-1 md:grid-cols-3 gap-5 mb-20">
      
      <!-- What is Starvation — with mini timeline -->
      <div class="bg-white/5 border border-white/10 rounded-xl p-6 hover:border-red-500/30 hover:bg-red-500/5 transition-all duration-300">
        <div class="flex items-center gap-3 mb-4">
          <div class="p-2 bg-red-500/10 rounded-lg text-red-400"><AlertTriangle size={20} /></div>
          <h3 class="font-mono text-sm tracking-widest text-red-400 uppercase">What is Starvation?</h3>
        </div>
        <!-- Mini timeline visualization -->
        <div class="mb-4 bg-black/30 rounded-lg p-3 border border-white/5">
          <div class="font-mono text-[9px] text-slate-600 mb-2">LOCK ACQUISITION TIMELINE</div>
          <div class="space-y-1.5">
            <div class="flex items-center gap-2">
              <span class="w-12 text-xs font-mono text-emerald-400">T_greedy</span>
              <div class="flex-1 flex gap-0.5">
                {#each Array(12) as _, i}
                  <div class="h-3 flex-1 rounded-sm {i % 3 !== 2 ? 'bg-emerald-500' : 'bg-emerald-500/30'}"></div>
                {/each}
              </div>
            </div>
            <div class="flex items-center gap-2">
              <span class="w-12 text-xs font-mono text-red-400">T_starved</span>
              <div class="flex-1 flex gap-0.5">
                {#each Array(12) as _, i}
                  <div class="h-3 flex-1 rounded-sm {i === 5 ? 'bg-amber-500' : 'bg-red-500/20'}"></div>
                {/each}
              </div>
            </div>
          </div>
          <div class="flex justify-between mt-2 font-mono text-[10px] text-slate-600">
            <span>t=0</span>
            <span class="text-red-400">1 write in 12 ticks ↑</span>
            <span>t=12</span>
          </div>
        </div>
        <p class="text-sm text-slate-400 leading-relaxed">
          A thread <span class="text-white">repeatedly fails</span> to acquire the lock. It's alive but can never make progress — <span class="text-red-400">starved</span>.
        </p>
      </div>

      <!-- Why — with visual diagram -->
      <div class="bg-white/5 border border-white/10 rounded-xl p-6 hover:border-amber-500/30 hover:bg-amber-500/5 transition-all duration-300">
        <div class="flex items-center gap-3 mb-4">
          <div class="p-2 bg-amber-500/10 rounded-lg text-amber-400"><Users size={20} /></div>
          <h3 class="font-mono text-sm tracking-widest text-amber-400 uppercase">Why It Happens</h3>
        </div>
        <!-- Cycle diagram -->
        <div class="mb-4 bg-black/30 rounded-lg p-3 border border-white/5">
          <div class="font-mono text-[10px] text-slate-600 mb-2">GREEDY RE-ACQUISITION CYCLE</div>
          <div class="flex items-center justify-center gap-2 py-2">
            <div class="flex flex-col items-center gap-1">
              <div class="w-10 h-10 rounded-lg bg-amber-500/20 border border-amber-500/30 flex items-center justify-center text-xs font-mono text-amber-400">LOCK</div>
            </div>
            <div class="text-amber-400 text-lg">→</div>
            <div class="flex flex-col items-center gap-1">
              <div class="w-10 h-10 rounded-lg bg-emerald-500/20 border border-emerald-500/30 flex items-center justify-center text-xs font-mono text-emerald-400">WRITE</div>
            </div>
            <div class="text-amber-400 text-lg">→</div>
            <div class="flex flex-col items-center gap-1">
              <div class="w-10 h-10 rounded-lg bg-slate-500/20 border border-slate-500/30 flex items-center justify-center text-xs font-mono text-slate-400">FREE</div>
            </div>
            <div class="text-red-400 text-lg font-bold">↻</div>
          </div>
          <div class="text-center font-mono text-[10px] text-red-400 mt-1">
            same thread re-acquires before others wake up
          </div>
        </div>
        <p class="text-sm text-slate-400 leading-relaxed">
          <span class="text-white">Unfair scheduling</span> — the OS doesn't guarantee order. A thread that releases can <span class="text-amber-400">instantly re-acquire</span>.
        </p>
      </div>

      <!-- Rust's Solution — with shield graphic -->
      <div class="bg-white/5 border border-white/10 rounded-xl p-6 hover:border-emerald-500/30 hover:bg-emerald-500/5 transition-all duration-300">
        <div class="flex items-center gap-3 mb-4">
          <div class="p-2 bg-emerald-500/10 rounded-lg text-emerald-400"><Shield size={20} /></div>
          <h3 class="font-mono text-sm tracking-widest text-emerald-400 uppercase">How Rust Solves It</h3>
        </div>
        <!-- Rust guarantees visual -->
        <div class="mb-4 bg-black/30 rounded-lg p-3 border border-white/5 space-y-2">
          <div class="font-mono text-[10px] text-slate-600 mb-1">RUST'S SAFETY LAYERS</div>
          <div class="flex items-center gap-2">
            <div class="w-4 h-4 rounded-full bg-emerald-500 flex items-center justify-center text-[8px] text-black font-bold">✓</div>
            <span class="text-xs font-mono text-slate-300">Ownership → no data races at <span class="text-emerald-400">compile time</span></span>
          </div>
          <div class="flex items-center gap-2">
            <div class="w-4 h-4 rounded-full bg-emerald-500 flex items-center justify-center text-[8px] text-black font-bold">✓</div>
            <span class="text-xs font-mono text-slate-300">MutexGuard → <span class="text-emerald-400">auto-unlock</span> on scope exit</span>
          </div>
          <div class="flex items-center gap-2">
            <div class="w-4 h-4 rounded-full bg-emerald-500 flex items-center justify-center text-[8px] text-black font-bold">✓</div>
            <span class="text-xs font-mono text-slate-300">Poison detection → handles <span class="text-emerald-400">panicked</span> threads</span>
          </div>
          <div class="flex items-center gap-2">
            <div class="w-4 h-4 rounded-full bg-emerald-500 flex items-center justify-center text-[8px] text-black font-bold">✓</div>
            <span class="text-xs font-mono text-slate-300">parking_lot → true <span class="text-emerald-400">FIFO</span> fair queue</span>
          </div>
        </div>
        <p class="text-sm text-slate-400 leading-relaxed">
          Can't forget to unlock, can't access data without the lock. <span class="text-emerald-400">The compiler catches it.</span>
        </p>
      </div>
    </div>

    <!-- ═══════════════════════════════════ -->
    <!-- ARC DEEP DIVE — Toggle View        -->
    <!-- ═══════════════════════════════════ -->
    <div class="section-anim opacity-0 translate-y-6 mb-20">
      <div class="flex flex-col md:flex-row items-start md:items-center justify-between gap-4 mb-6">
        <div class="flex items-center gap-3">
          <Share2 size={18} class="text-cyan-400" />
          <h2 class="font-mono text-sm tracking-widest text-cyan-400 uppercase">Understanding Arc&lt;Mutex&lt;T&gt;&gt;</h2>
        </div>
        <!-- Toggle -->
        <div class="flex items-center gap-1 bg-white/5 border border-white/10 rounded-lg p-1">
          <button on:click={() => arcView = "simple"}
            class="px-4 py-2 rounded-md text-xs font-mono tracking-wider transition-all duration-200
              {arcView === 'simple' ? 'bg-cyan-500/20 text-cyan-400 border border-cyan-500/30' : 'text-slate-500 hover:text-slate-300 border border-transparent'}">
            Simple
          </button>
          <button on:click={() => arcView = "technical"}
            class="px-4 py-2 rounded-md text-xs font-mono tracking-wider transition-all duration-200
              {arcView === 'technical' ? 'bg-cyan-500/20 text-cyan-400 border border-cyan-500/30' : 'text-slate-500 hover:text-slate-300 border border-transparent'}">
            Technical
          </button>
        </div>
      </div>

      <!-- Single panel that switches -->
      <div class="bg-white/[0.03] border border-cyan-500/20 rounded-xl overflow-hidden mb-6">
        {#if arcView === "simple"}
          <!-- Simple / Layman -->
          <div class="flex items-center gap-2 px-5 py-3 border-b border-white/5 bg-cyan-500/5">
            <Eye size={14} class="text-cyan-400" />
            <span class="font-mono text-xs text-cyan-400 tracking-widest uppercase">The Simple Version</span>
          </div>
          <div class="p-6">
            <div class="bg-black/30 rounded-lg p-4 border border-white/5 mb-5">
              <div class="font-mono text-[9px] text-slate-600 mb-3">THE LIBRARY BOOK ANALOGY</div>
              <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                <div class="flex items-center gap-3">
                  <div class="w-10 h-10 rounded-lg bg-cyan-500/20 border border-cyan-500/30 flex items-center justify-center">
                    <Share2 size={16} class="text-cyan-400" />
                  </div>
                  <div>
                    <div class="text-xs font-mono text-cyan-400">Arc = Library Card</div>
                    <div class="text-[10px] text-slate-500">Multiple readers, one book</div>
                  </div>
                </div>
                <div class="flex items-center gap-3">
                  <div class="w-10 h-10 rounded-lg bg-amber-500/20 border border-amber-500/30 flex items-center justify-center">
                    <Lock size={16} class="text-amber-400" />
                  </div>
                  <div>
                    <div class="text-xs font-mono text-amber-400">Mutex = Sign-out Sheet</div>
                    <div class="text-[10px] text-slate-500">Only one person writes at a time</div>
                  </div>
                </div>
                <div class="flex items-center gap-3">
                  <div class="w-10 h-10 rounded-lg bg-emerald-500/20 border border-emerald-500/30 flex items-center justify-center">
                    <Shield size={16} class="text-emerald-400" />
                  </div>
                  <div>
                    <div class="text-xs font-mono text-emerald-400">MutexGuard = Receipt</div>
                    <div class="text-[10px] text-slate-500">Auto-returns when done</div>
                  </div>
                </div>
              </div>
            </div>
            <p class="text-sm text-slate-400 leading-relaxed">
              Think of <span class="text-cyan-400 font-semibold">Arc</span> as a <span class="text-white">shared library card</span>. 
              Multiple people (threads) each get their own copy of the card. The card doesn't hold the book — 
              it just knows <span class="text-white">where the book is</span> and <span class="text-cyan-400">how many people are using it</span>.
            </p>
            <p class="text-sm text-slate-400 leading-relaxed mt-3">
              The <span class="text-amber-400 font-semibold">Mutex</span> is the <span class="text-white">sign-out sheet</span> — 
              only one person can check out the book at a time. When you're done, you drop the receipt and it auto-returns. 
              <span class="text-emerald-400">You can't forget to return it.</span>
            </p>
          </div>
        {:else}
          <!-- Technical / Professional -->
          <div class="flex items-center gap-2 px-5 py-3 border-b border-white/5 bg-cyan-500/5">
            <GraduationCap size={14} class="text-cyan-400" />
            <span class="font-mono text-xs text-cyan-400 tracking-widest uppercase">The Technical Version</span>
          </div>
          <div class="p-6">
            <div class="bg-black/30 rounded-lg p-4 border border-white/5 mb-5">
              <div class="font-mono text-[10px] text-slate-600 mb-3">ARC REFERENCE COUNTING</div>
              <svg viewBox="0 0 200 120" class="w-full" style="height: 220px;">
                <!-- Central Mutex box -->
                <rect x="70" y="42" width="60" height="22" rx="4" fill="rgba(34,211,238,0.15)" stroke="rgba(34,211,238,0.4)" stroke-width="0.7" />
                <text x="100" y="56" text-anchor="middle" fill="#22d3ee" font-size="8" font-family="monospace">Mutex&lt;T&gt;</text>
                
                <!-- Ref count badge -->
                <rect x="80" y="30" width="40" height="12" rx="3" fill="rgba(34,211,238,0.2)" stroke="rgba(34,211,238,0.3)" stroke-width="0.5" />
                <text x="100" y="39" text-anchor="middle" fill="#22d3ee" font-size="7" font-family="monospace">refs: 4</text>

                <!-- Thread nodes + connecting lines -->
                {#each arcNodes as node, i}
                  <line x1="100" y1={i === 0 ? 30 : 64} x2={node.x} y2={node.y - 4} stroke="rgba(34,211,238,0.25)" stroke-width="0.6" stroke-dasharray="3,2" />
                  <rect x={node.x - 22} y={node.y - 7} width="44" height="14" rx="3" fill="rgba(255,255,255,0.06)" stroke="rgba(255,255,255,0.2)" stroke-width="0.4" />
                  <text x={node.x} y={node.y + 3} text-anchor="middle" fill={i === 0 ? '#22d3ee' : threadColors[i - 1]} font-size="6" font-family="monospace">{node.label}</text>
                {/each}
              </svg>
            </div>
            <div class="space-y-3">
              <div class="flex items-start gap-3">
                <span class="text-cyan-400 font-mono text-xs mt-0.5 shrink-0">Arc</span>
                <p class="text-sm text-slate-400"><span class="text-white">Atomically Reference Counted</span> smart pointer. Uses <code class="text-cyan-400 text-xs">AtomicUsize</code> under the hood — each <code class="text-cyan-400 text-xs">clone()</code> increments the count atomically, each <code class="text-cyan-400 text-xs">drop()</code> decrements it. When count hits 0 → data is deallocated. Thread-safe, unlike <code class="text-cyan-400 text-xs">Rc</code>.</p>
              </div>
              <div class="flex items-start gap-3">
                <span class="text-amber-400 font-mono text-xs mt-0.5 shrink-0">Mutex</span>
                <p class="text-sm text-slate-400">Wraps data with a lock. <code class="text-amber-400 text-xs">.lock()</code> returns <code class="text-amber-400 text-xs">Result&lt;MutexGuard&lt;T&gt;&gt;</code> — a RAII guard that <code class="text-amber-400 text-xs">Deref</code>s to <code class="text-amber-400 text-xs">&T</code> / <code class="text-amber-400 text-xs">&mut T</code> and releases the lock on drop.</p>
              </div>
            </div>
          </div>
        {/if}
      </div>

      <!-- Arc lifecycle visual -->
      <div class="bg-white/[0.03] border border-white/5 rounded-xl p-5">
        <div class="font-mono text-[9px] text-slate-600 mb-3">ARC LIFECYCLE — WHAT HAPPENS STEP BY STEP</div>
        <div class="flex flex-wrap items-center gap-3 justify-center">
          {#each [
            { step: "1", label: "Arc::new(Mutex::new(0))", color: "cyan", note: "refs = 1" },
            { step: "2", label: "Arc::clone(&counter)", color: "emerald", note: "refs = 2" },
            { step: "3", label: "move || { ... }", color: "amber", note: "clone sent to thread" },
            { step: "4", label: "c.lock().unwrap()", color: "amber", note: "MutexGuard acquired" },
            { step: "5", label: "*num += 1", color: "emerald", note: "mutate inner data" },
            { step: "6", label: "} // scope ends", color: "cyan", note: "guard dropped → unlock" },
            { step: "7", label: "thread exits", color: "slate", note: "Arc dropped → refs - 1" },
          ] as item}
            <div class="flex items-center gap-2 bg-black/30 rounded-lg px-3 py-2 border border-white/5">
              <div class="w-5 h-5 rounded-full bg-{item.color}-500/20 border border-{item.color}-500/30 flex items-center justify-center text-[9px] font-mono text-{item.color}-400 shrink-0">{item.step}</div>
              <div>
                <div class="text-[10px] font-mono text-white">{item.label}</div>
                <div class="text-[8px] font-mono text-slate-500">{item.note}</div>
              </div>
            </div>
          {/each}
        </div>
      </div>
    </div>

    <!-- ═══════════════════════════════════ -->
    <!-- LIVE SIMULATION                    -->
    <!-- ═══════════════════════════════════ -->
    <div class="section-anim opacity-0 translate-y-6 mb-20">
      
      <div class="flex flex-col md:flex-row items-start md:items-center justify-between gap-4 mb-6">
        <div>
          <div class="flex items-center gap-3 mb-2">
            <Gauge size={18} class="text-emerald-400" />
            <h2 class="font-mono text-sm tracking-widest text-emerald-400 uppercase">Live Simulation</h2>
          </div>
          <p class="text-base text-slate-500">Watch threads compete for a single mutex in real time.</p>
        </div>
        
        <div class="flex items-center gap-1 bg-white/5 border border-white/10 rounded-lg p-1">
          <button on:click={() => toggleMode("unfair")}
            class="px-4 py-2 rounded-md text-xs font-mono tracking-wider transition-all duration-200
              {mode === 'unfair' ? 'bg-red-500/20 text-red-400 border border-red-500/30' : 'text-slate-500 hover:text-slate-300 border border-transparent'}">
            Unfair
          </button>
          <button on:click={() => toggleMode("fair")}
            class="px-4 py-2 rounded-md text-xs font-mono tracking-wider transition-all duration-200
              {mode === 'fair' ? 'bg-emerald-500/20 text-emerald-400 border border-emerald-500/30' : 'text-slate-500 hover:text-slate-300 border border-transparent'}">
            Rust Fair
          </button>
        </div>
      </div>

      <div class="bg-white/[0.03] border border-white/10 rounded-2xl overflow-hidden">
        
        <!-- Controls -->
        <div class="flex flex-wrap items-center justify-between gap-4 px-6 py-4 border-b border-white/5 bg-black/30">
          <div class="flex items-center gap-3">
            <button on:click={() => running ? stopSim() : startSim()}
              class="flex items-center gap-2 px-4 py-2 rounded-lg text-xs font-mono tracking-wider transition-all duration-200
                {running ? 'bg-amber-500/20 text-amber-400 border border-amber-500/30 hover:bg-amber-500/30' : 'bg-emerald-500/20 text-emerald-400 border border-emerald-500/30 hover:bg-emerald-500/30'}">
              {#if running}<Pause size={14} /> PAUSE{:else}<Play size={14} /> START{/if}
            </button>
            <button on:click={resetSim}
              class="flex items-center gap-2 px-3 py-2 rounded-lg text-xs font-mono text-slate-500 hover:text-white border border-white/5 hover:border-white/20 transition-all">
              <RotateCcw size={14} /> RESET
            </button>
          </div>
          <div class="flex items-center gap-6">
            <div class="flex items-center gap-2">
              <span class="font-mono text-[10px] text-slate-500 uppercase tracking-widest">Threads</span>
              <input type="range" min="2" max="8" bind:value={threadCount} on:change={resetSim}
                class="w-20 h-1 accent-emerald-500 bg-white/10 rounded-full appearance-none cursor-pointer" />
              <span class="font-mono text-xs text-white w-4">{threadCount}</span>
            </div>
            <div class="flex items-center gap-2">
              <span class="font-mono text-[10px] text-slate-500 uppercase tracking-widest">Speed</span>
              <input type="range" min="1" max="5" bind:value={speed}
                class="w-20 h-1 accent-emerald-500 bg-white/10 rounded-full appearance-none cursor-pointer" />
              <span class="font-mono text-xs text-white w-4">{speed}x</span>
            </div>
          </div>
        </div>

        <!-- Metrics -->
        <div class="flex flex-wrap items-center gap-6 px-6 py-3 border-b border-white/5 bg-black/20">
          <div class="flex items-center gap-2">
            <Lock size={12} class="text-slate-500" />
            <span class="font-mono text-[10px] text-slate-500 uppercase tracking-widest">Mode</span>
            <span class="font-mono text-xs {mode === 'unfair' ? 'text-red-400' : 'text-emerald-400'}">{mode === 'unfair' ? 'UNFAIR' : 'FAIR (FIFO)'}</span>
          </div>
          <div class="hidden md:block w-px h-4 bg-white/10"></div>
          <div class="flex items-center gap-2">
            <span class="font-mono text-xs text-slate-500 uppercase tracking-widest">Mode</span>
            <span class="font-mono text-sm {mode === 'unfair' ? 'text-red-400' : 'text-emerald-400'}">{mode === 'unfair' ? 'UNFAIR' : 'FAIR (FIFO)'}</span>
          </div>
          <div class="hidden md:block w-px h-4 bg-white/10"></div>
          <div class="flex items-center gap-2">
            <span class="font-mono text-xs text-slate-500 uppercase tracking-widest">Counter</span>
            <span class="font-mono text-sm text-white">{sharedCounter}</span>
          </div>
          <div class="hidden md:block w-px h-4 bg-white/10"></div>
          <div class="flex items-center gap-2">
            <span class="font-mono text-xs text-slate-500 uppercase tracking-widest">Starvation Index</span>
            <span class="font-mono text-sm font-bold {Number(starvationIndex) > 3 ? 'text-red-400' : Number(starvationIndex) > 1.5 ? 'text-amber-400' : 'text-emerald-400'}">
              {starvationIndex}x
            </span>
            {#if Number(starvationIndex) > 5}
              <span class="text-xs text-red-500 animate-pulse font-mono">CRITICAL</span>
            {/if}
          </div>
          <div class="hidden md:block w-px h-4 bg-white/10"></div>
          <div class="flex items-center gap-2">
            <span class="font-mono text-[10px] text-slate-500 uppercase tracking-widest">Ticks</span>
            <span class="font-mono text-xs text-white">{tick}</span>
          </div>
        </div>

        <!-- Thread Bars + Live Graph Side by Side -->
        <div class="grid grid-cols-1 lg:grid-cols-5 gap-0">
          
          <!-- Thread Bars (3/5 width) -->
          <div class="lg:col-span-3 p-6 space-y-3 border-b lg:border-b-0 lg:border-r border-white/5">
            {#each threads as thread (thread.id)}
              <div class="flex items-center gap-3">
                <div class="w-24 md:w-32 shrink-0 flex items-center gap-2">
                  <div class="w-2 h-2 rounded-full {getStateDot(thread.state)} {thread.state === 'STARVING' ? 'animate-pulse' : ''}"></div>
                  <span class="font-mono text-sm text-slate-400 truncate">{thread.name}</span>
                </div>
                <div class="flex-1 h-7 bg-white/5 rounded-md overflow-hidden relative border border-white/5">
                  <div class="h-full rounded-md transition-all duration-200 ease-out {getStateColor(thread.state)} {thread.state === 'STARVING' ? 'animate-pulse' : ''}"
                    style="width: {maxWrites > 0 ? (thread.writes / maxWrites) * 100 : 0}%"></div>
                  <div class="absolute inset-0 flex items-center px-3">
                    <span class="font-mono text-xs text-white/80 drop-shadow-lg">{thread.writes}</span>
                  </div>
                </div>
                <div class="w-20 shrink-0 text-right">
                  <span class="font-mono text-[10px] tracking-wider {getStateTextColor(thread.state)} {thread.state === 'STARVING' ? 'font-bold' : ''}">
                    {thread.state}
                  </span>
                </div>
              </div>
            {/each}
          </div>

          <!-- Live Write Graph (2/5 width) -->
          <div class="lg:col-span-2 p-4">
            <div class="flex items-center gap-2 mb-3">
              <TrendingUp size={14} class="text-slate-500" />
              <span class="font-mono text-[10px] text-slate-500 uppercase tracking-widest">Write Distribution Over Time</span>
            </div>
            <div class="bg-black/30 rounded-lg border border-white/5 p-3 relative" style="height: 160px;">
              <svg viewBox="0 0 100 100" preserveAspectRatio="none" class="w-full h-full">
                <!-- Grid lines -->
                {#each [25, 50, 75] as y}
                  <line x1="0" y1={y} x2="100" y2={y} stroke="rgba(255,255,255,0.04)" stroke-width="0.3" />
                {/each}
                <!-- Thread lines -->
                {#each threads as thread, i}
                  {#if writeHistory[i] && writeHistory[i].length > 1}
                    <path d={getGraphPath(i)} fill="none" stroke={threadColors[i]} stroke-width="1" opacity="0.8" />
                  {/if}
                {/each}
              </svg>
              {#if tick < 2}
                <div class="absolute inset-0 flex items-center justify-center">
                  <span class="font-mono text-[10px] text-slate-600">Start simulation to see graph →</span>
                </div>
              {/if}
            </div>
            <!-- Legend -->
            <div class="flex flex-wrap gap-x-3 gap-y-1 mt-2">
              {#each threads as thread, i}
                <div class="flex items-center gap-1">
                  <div class="w-2 h-2 rounded-full" style="background-color: {threadColors[i]}"></div>
                  <span class="font-mono text-[9px] text-slate-500">{thread.name}</span>
                </div>
              {/each}
            </div>
          </div>
        </div>

        <!-- Shared Resource + Write Share Chart -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-0 border-t border-white/5">
          
          <!-- Shared Resource -->
          <div class="p-5 border-b md:border-b-0 md:border-r border-white/5">
            <div class="flex items-center justify-between">
              <div class="flex items-center gap-3">
                {#if mode === "unfair"}<Unlock size={18} class="text-red-400" />{:else}<Lock size={18} class="text-emerald-400" />{/if}
                <div>
                  <div class="font-mono text-[10px] text-slate-500 uppercase tracking-widest">Shared Resource</div>
                  <div class="font-mono text-xl text-white">counter = <span class="{mode === 'unfair' ? 'text-red-400' : 'text-emerald-400'}">{sharedCounter}</span></div>
                </div>
              </div>
            </div>
            {#if mode === "unfair" && running && Number(starvationIndex) > 3}
              <div class="mt-3 flex items-center gap-2 text-red-400 font-mono text-xs border-t border-white/5 pt-3">
                <AlertTriangle size={14} class="animate-pulse shrink-0" />
                <span>"{threads[0]?.name}" is monopolizing — others are starving!</span>
              </div>
            {/if}
            {#if mode === "fair" && running && tick > 10}
              <div class="mt-3 flex items-center gap-2 text-emerald-400 font-mono text-xs border-t border-white/5 pt-3">
                <Shield size={14} class="shrink-0" />
                <span>FIFO ensures equal access — no starvation possible.</span>
              </div>
            {/if}
          </div>

          <!-- Write Share Bar Chart -->
          <div class="p-5">
            <div class="flex items-center gap-2 mb-3">
              <BarChart3 size={14} class="text-slate-500" />
              <span class="font-mono text-[10px] text-slate-500 uppercase tracking-widest">Write Share %</span>
            </div>
            <div class="space-y-1.5">
              {#each threads as thread, i}
                <div class="flex items-center gap-2">
                  <span class="w-14 font-mono text-[9px] text-slate-500 truncate">{thread.name}</span>
                  <div class="flex-1 h-4 bg-white/5 rounded-sm overflow-hidden">
                    <div class="h-full rounded-sm transition-all duration-300" 
                      style="width: {writeShares[i]}%; background-color: {threadColors[i]}; opacity: 0.7;"></div>
                  </div>
                  <span class="w-10 text-right font-mono text-[9px] text-slate-400">{writeShares[i]}%</span>
                </div>
              {/each}
            </div>
          </div>
        </div>

        <!-- Status Legend -->
        <div class="px-6 py-3 border-t border-white/5 flex flex-wrap gap-4">
          <div class="flex items-center gap-2"><div class="w-3 h-3 rounded-sm bg-emerald-500"></div><span class="font-mono text-[10px] text-slate-500">WRITING</span></div>
          <div class="flex items-center gap-2"><div class="w-3 h-3 rounded-sm bg-amber-500"></div><span class="font-mono text-[10px] text-slate-500">ACQUIRING</span></div>
          <div class="flex items-center gap-2"><div class="w-3 h-3 rounded-sm bg-red-500/60"></div><span class="font-mono text-[10px] text-slate-500">BLOCKED</span></div>
          <div class="flex items-center gap-2"><div class="w-3 h-3 rounded-sm bg-red-600 animate-pulse"></div><span class="font-mono text-[10px] text-slate-500">STARVING</span></div>
        </div>
      </div>
    </div>

    <!-- ═══════════════════════════════════ -->
    <!-- CODE COMPARISON                    -->
    <!-- ═══════════════════════════════════ -->
    <div class="section-anim opacity-0 translate-y-6 mb-20">
      <div class="flex items-center gap-3 mb-6">
        <Code2 size={18} class="text-slate-500" />
        <h2 class="font-mono text-sm tracking-widest text-slate-500 uppercase">Code Comparison</h2>
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-2 gap-5">
        <!-- C/Pthreads -->
        <div class="bg-white/[0.03] border border-red-500/20 rounded-xl overflow-hidden">
          <div class="flex items-center gap-2 px-5 py-3 border-b border-white/5 bg-red-500/5">
            <div class="w-2 h-2 rounded-full bg-red-400"></div>
            <span class="font-mono text-xs text-red-400 tracking-widest uppercase">C / Pthreads — No Fairness</span>
          </div>
          <pre class="p-5 text-sm font-mono leading-relaxed overflow-x-auto text-slate-300"><span class="text-slate-500">// No ordering guarantee — OS picks the winner</span>
<span class="text-red-400">pthread_mutex_t</span> lock = PTHREAD_MUTEX_INITIALIZER;
<span class="text-amber-400">int</span> shared_counter = <span class="text-emerald-400">0</span>;

<span class="text-amber-400">void</span>* <span class="text-white">writer_thread</span>(<span class="text-amber-400">void</span>* arg) &#123;
    <span class="text-amber-400">while</span> (<span class="text-emerald-400">1</span>) &#123;
        <span class="text-red-400">pthread_mutex_lock</span>(&lock);
        <span class="text-slate-500">// WARN: Just released? OS may let me re-acquire</span>
        <span class="text-slate-500">// before other threads even wake up!</span>
        shared_counter++;
        <span class="text-red-400">pthread_mutex_unlock</span>(&lock);
        <span class="text-slate-500">// ^ Greedy thread runs again immediately</span>
    &#125;
    <span class="text-amber-400">return</span> NULL;
&#125;

<span class="text-slate-500">// [X] No FIFO guarantee</span>
<span class="text-slate-500">// [X] No compile-time data race protection</span>
<span class="text-slate-500">// [X] Forgetting unlock → deadlock</span>
<span class="text-slate-500">// [X] No poison detection for panics</span></pre>
        </div>

        <!-- Rust -->
        <div class="bg-white/[0.03] border border-emerald-500/20 rounded-xl overflow-hidden">
          <div class="flex items-center gap-2 px-5 py-3 border-b border-white/5 bg-emerald-500/5">
            <div class="w-2 h-2 rounded-full bg-emerald-400"></div>
            <span class="font-mono text-xs text-emerald-400 tracking-widest uppercase">Rust — Ownership + Fair Mutex</span>
          </div>
          <pre class="p-5 text-sm font-mono leading-relaxed overflow-x-auto text-slate-300"><span class="text-amber-400">use</span> std::sync::&#123;Arc, Mutex&#125;;
<span class="text-amber-400">use</span> std::thread;

<span class="text-amber-400">fn</span> <span class="text-white">main</span>() &#123;
    <span class="text-slate-500">// Arc = atomic ref count, Mutex = mutual exclusion</span>
    <span class="text-amber-400">let</span> counter = <span class="text-emerald-400">Arc::new</span>(<span class="text-emerald-400">Mutex::new</span>(<span class="text-emerald-400">0</span>));
    <span class="text-amber-400">let mut</span> handles = <span class="text-emerald-400">vec!</span>[];

    <span class="text-amber-400">for</span> _ <span class="text-amber-400">in</span> <span class="text-emerald-400">0</span>..<span class="text-emerald-400">4</span> &#123;
        <span class="text-amber-400">let</span> c = <span class="text-emerald-400">Arc::clone</span>(&counter);
        handles.<span class="text-emerald-400">push</span>(thread::<span class="text-emerald-400">spawn</span>(<span class="text-amber-400">move</span> || &#123;
            <span class="text-amber-400">let mut</span> num = c.<span class="text-emerald-400">lock</span>().<span class="text-emerald-400">unwrap</span>();
            <span class="text-slate-500">// ✅ MutexGuard auto-drops at scope end</span>
            <span class="text-slate-500">// ✅ Can't access data without the lock</span>
            *num += <span class="text-emerald-400">1</span>;
        &#125;));
    &#125;

    <span class="text-amber-400">for</span> h <span class="text-amber-400">in</span> handles &#123;
        h.<span class="text-emerald-400">join</span>().<span class="text-emerald-400">unwrap</span>();
    &#125;
&#125;
<span class="text-slate-500">// ✅ Compile-time data race prevention</span>
<span class="text-slate-500">// ✅ Auto-unlock via RAII (MutexGuard)</span>
<span class="text-slate-500">// ✅ Poison detection on thread panic</span></pre>
        </div>
      </div>
    </div>

    <!-- ═══════════════════════════════════ -->
    <!-- TAKEAWAY                           -->
    <!-- ═══════════════════════════════════ -->
    <div class="section-anim opacity-0 translate-y-6 border border-emerald-500/20 bg-emerald-500/5 rounded-xl p-6 relative overflow-hidden mb-12">
      <div class="absolute inset-0 bg-gradient-to-r from-transparent via-emerald-500/5 to-transparent opacity-50"></div>
      <div class="relative z-10 flex items-start gap-4">
        <div class="p-2 bg-emerald-500/10 rounded-lg text-emerald-400 shrink-0 mt-1">
          <Lightbulb size={20} />
        </div>
        <div>
          <h3 class="text-emerald-400 font-mono text-sm font-bold uppercase tracking-widest mb-3">The Takeaway</h3>
          <div class="grid grid-cols-1 md:grid-cols-3 gap-4 text-sm text-slate-300">
            <div>
              <span class="text-white font-semibold block mb-1">Starvation is real</span>
              It happens in production under load — not just in textbooks. The simulation above proves it.
            </div>
            <div>
              <span class="text-white font-semibold block mb-1">Ownership = Zero-cost safety</span>
              You can't access shared data without the lock. The lock auto-releases via RAII. The compiler enforces it.
            </div>
            <div>
              <span class="text-white font-semibold block mb-1">Arc makes sharing safe</span>
              Thread-safe reference counting lets multiple threads share ownership. Combined with Mutex — safe concurrent mutation.
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Footer -->
    <div class="section-anim opacity-0 translate-y-6 border-t border-white/10 pt-6">
      <div class="font-mono text-xs text-slate-500 flex flex-col md:flex-row justify-between gap-4">
        <div class="flex items-center gap-2">
          <span class="text-emerald-400">▶</span>
          <span>experiment::lock_contention — simulation complete</span>
        </div>
        <div class="flex gap-6 opacity-50">
          <span>MODE: {mode.toUpperCase()}</span>
          <span>THREADS: {threadCount}</span>
          <span>STATUS: <span class="{running ? 'text-emerald-400' : 'text-slate-400'}">{running ? 'RUNNING' : 'IDLE'}</span></span>
        </div>
      </div>
    </div>

  </div>
</div>

<style>
  input[type="range"]::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    width: 14px;
    height: 14px;
    border-radius: 50%;
    background: #34d399;
    cursor: pointer;
    border: 2px solid #0a0a0a;
  }
  input[type="range"]::-moz-range-thumb {
    width: 14px;
    height: 14px;
    border-radius: 50%;
    background: #34d399;
    cursor: pointer;
    border: 2px solid #0a0a0a;
  }
  pre {
    scrollbar-width: thin;
    scrollbar-color: rgba(255,255,255,0.1) transparent;
  }
</style>
