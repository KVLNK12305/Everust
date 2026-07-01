<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { gsap } from "gsap";
  import { 
    ArrowLeft, Cpu, Lock, Shield, Sliders, Zap, Terminal, AlertCircle, 
    Play, Pause, RefreshCw, Check, ChevronRight, Activity, Layers, Code
  } from "lucide-svelte";

  // ─── Navigation & Active Section ───
  let activeSection = 0;

  const sections = [
    { id: "slice", num: "01", title: "Zero-Copy Pipeline", sub: "&[T] Slices vs Vec<T>", icon: Cpu },
    { id: "zeroize", num: "02", title: "Active Zeroization", sub: "Volatile Memory Wiping", icon: Lock },
    { id: "verdict", num: "03", title: "Exhaustive Verdicts", sub: "Algebraic Data Types", icon: Shield },
    { id: "serde", num: "04", title: "Resilient Configs", sub: "Serde Default Fallbacks", icon: Sliders },
    { id: "tokio", num: "05", title: "Async Actor Loops", sub: "Work-Stealing Runtime", icon: Zap },
    { id: "tracing", num: "06", title: "Structured Telemetry", sub: "JSON vs Synchronous Print", icon: Terminal },
    { id: "error", num: "07", title: "Error Propagation", sub: "thiserror + anyhow Chain", icon: AlertCircle }
  ];

  // ─── 1. Zero-Copy Slice Lab State ───
  let simMode: "vec" | "slice" = "slice";
  let allocCount = 0;
  let heapMemoryKb = 0;
  let opsPerSec = 24500;
  let latencyUs = 0.42;
  let isScanning = false;
  let scanTimer: ReturnType<typeof setInterval> | null = null;
  let streamLog: { id: number; text: string; latency: string; mode: string }[] = [];

  function toggleScan() {
    if (isScanning) {
      if (scanTimer) clearInterval(scanTimer);
      isScanning = false;
    } else {
      isScanning = true;
      scanTimer = setInterval(() => {
        if (simMode === "vec") {
          allocCount += 15;
          heapMemoryKb += 48;
          opsPerSec = 3400 + Math.floor(Math.random() * 300);
          latencyUs = 14.8 + Math.random() * 1.5;
          if (streamLog.length > 5) streamLog.shift();
          streamLog = [...streamLog, { id: Math.random(), text: "Heap allocation & clone executed", latency: `${latencyUs.toFixed(1)} µs`, mode: "vec" }];
        } else {
          allocCount += 0;
          heapMemoryKb = 0;
          opsPerSec = 28500 + Math.floor(Math.random() * 800);
          latencyUs = 0.38 + Math.random() * 0.05;
          if (streamLog.length > 5) streamLog.shift();
          streamLog = [...streamLog, { id: Math.random(), text: "Direct slice pointer bounds validation", latency: `${latencyUs.toFixed(2)} µs`, mode: "slice" }];
        }
      }, 250);
    }
  }

  function resetSim() {
    if (scanTimer) clearInterval(scanTimer);
    isScanning = false;
    allocCount = 0;
    heapMemoryKb = 0;
    streamLog = [];
  }

  // ─── 2. Active Zeroization State ───
  let memCells = [
    { addr: "0x7F00", val: "4A", label: "AES_IV[0]" },
    { addr: "0x7F01", val: "8F", label: "AES_IV[1]" },
    { addr: "0x7F02", val: "2B", label: "SECRET_KEY" },
    { addr: "0x7F03", val: "90", label: "SECRET_KEY" },
    { addr: "0x7F04", val: "E1", label: "SECRET_KEY" },
    { addr: "0x7F05", val: "CC", label: "AUTH_TAG" },
    { addr: "0x7F06", val: "3D", label: "BEARER_TOK" },
    { addr: "0x7F07", val: "55", label: "BEARER_TOK" }
  ];
  let memState: "active" | "dropped_dead" | "zeroized" = "active";
  let wipeProgress = 0;

  function dropWithoutZeroize() {
    memState = "dropped_dead";
    wipeProgress = 0;
  }

  function dropWithZeroize() {
    memState = "zeroized";
    let step = 0;
    const interval = setInterval(() => {
      if (step < memCells.length) {
        memCells[step].val = "00";
        wipeProgress = Math.floor(((step + 1) / memCells.length) * 100);
        memCells = [...memCells];
        step++;
      } else {
        clearInterval(interval);
      }
    }, 60);
  }

  function resetMemory() {
    memState = "active";
    wipeProgress = 0;
    const orig = ["4A", "8F", "2B", "90", "E1", "CC", "3D", "55"];
    memCells = memCells.map((c, i) => ({ ...c, val: orig[i] }));
  }

  // ─── 3. Verdict Routing State ───
  let selectedThreat = { id: "THR-909", conf: 0.94, sev: "Critical", ip: "198.51.100.24", type: "Kernel Rootkit Signature" };
  let currentVerdict = "Action::Neutralize(NeutralizationMethod::Eliminate)";
  let verdictStyle = "border-red-500/30 bg-red-500/5 text-red-400";
  let historyLog: { id: string; sev: string; verdict: string; time: string }[] = [
    { id: "THR-882", sev: "High", verdict: "Neutralize(Isolate)", time: "17:44:02" },
    { id: "THR-881", sev: "Medium", verdict: "Probe", time: "17:43:55" },
    { id: "THR-880", sev: "Low", verdict: "Ignore", time: "17:43:12" }
  ];

  function injectThreat(sev: string, conf: number, ip: string, type: string) {
    const id = `THR-${Math.floor(100 + Math.random() * 900)}`;
    selectedThreat = { id, conf, sev, ip, type };
    
    let verdictStr = "";
    if (conf > 0.8 && sev === "Critical") {
      verdictStr = "Action::Neutralize(NeutralizationMethod::Eliminate)";
      verdictStyle = "border-red-500/30 bg-red-500/5 text-red-400";
    } else if (conf > 0.6 && sev === "High") {
      verdictStr = "Action::Neutralize(NeutralizationMethod::Isolate)";
      verdictStyle = "border-orange-500/30 bg-orange-500/5 text-orange-400";
    } else if (sev === "Medium") {
      verdictStr = "Action::Probe";
      verdictStyle = "border-yellow-500/30 bg-yellow-500/5 text-yellow-400";
    } else {
      verdictStr = "Action::Ignore";
      verdictStyle = "border-white/10 bg-white/5 text-slate-400";
    }
    currentVerdict = verdictStr;
    historyLog = [{ id, sev, verdict: verdictStr.replace("Action::", ""), time: new Date().toLocaleTimeString() }, ...historyLog.slice(0, 3)];
  }

  // ─── 4. Serde Resilient Config State ───
  let configScenario: "missing_agent" | "complete" | "corrupt" = "missing_agent";

  // ─── 5. Tokio Work-Stealing Core State ───
  let workerCores = [
    { id: 0, name: "Worker Core 0", load: 45, tasks: 124, status: "Active" },
    { id: 1, name: "Worker Core 1", load: 82, tasks: 310, status: "Executing" },
    { id: 2, name: "Worker Core 2", load: 58, tasks: 198, status: "Active" },
    { id: 3, name: "Worker Core 3", load: 18, tasks: 45, status: "Work Stealing" }
  ];
  let isTokioRunning = false;
  let tokioTimer: ReturnType<typeof setInterval> | null = null;
  let yieldRate = 8420;

  function toggleTokioSim() {
    if (isTokioRunning) {
      if (tokioTimer) clearInterval(tokioTimer);
      isTokioRunning = false;
    } else {
      isTokioRunning = true;
      tokioTimer = setInterval(() => {
        yieldRate = 8000 + Math.floor(Math.random() * 800);
        workerCores = workerCores.map(core => {
          const newLoad = Math.min(95, Math.max(15, Math.floor(core.load + (Math.random() * 24 - 12))));
          const status = newLoad > 75 ? "Executing" : (newLoad > 35 ? "Active" : "Work Stealing");
          return { ...core, load: newLoad, tasks: core.tasks + Math.floor(Math.random() * 8), status };
        });
      }, 350);
    }
  }

  // ─── 6. Tracing Telemetry State ───
  let logMode: "println" | "tracing_term" | "tracing_json" = "tracing_term";
  let logFilter: "info" | "debug" | "trace" = "debug";
  const sampleLogs = [
    { time: "17:45:53.860", level: "INFO", target: "ghostshell::perception", msg: "Scanning interface eth0 via XDP zero-copy bounds", threat_id: "THR-001", latency: "0.38µs" },
    { time: "17:45:53.861", level: "DEBUG", target: "ghostshell::actions", msg: "Evaluating verdict match tree", threat_id: "THR-001", conf: "0.95" },
    { time: "17:45:53.862", level: "INFO", target: "ghostshell::actions", msg: "Executing XDP-style neutralization", threat_id: "THR-001", method: "Eliminate" },
    { time: "17:45:53.864", level: "TRACE", target: "ghostshell::crypto", msg: "Scrubber invoked on Drop trait", addr: "0x7F00", status: "ZEROIZED" }
  ];

  // ─── 7. Dual-Layer Error State ───
  let errorScenario: "tamper" | "missing_file" | "socket" = "tamper";

  onMount(() => {
    const tl = gsap.timeline();
    tl.to(".fade-in-up", {
      y: 0,
      opacity: 1,
      stagger: 0.06,
      duration: 0.6,
      ease: "power2.out"
    });

    const handleKeydown = (e: KeyboardEvent) => {
      if (e.key >= "1" && e.key <= "7" && !e.metaKey && !e.ctrlKey) {
        activeSection = parseInt(e.key) - 1;
      }
    };
    window.addEventListener("keydown", handleKeydown);
    return () => window.removeEventListener("keydown", handleKeydown);
  });

  onDestroy(() => {
    if (scanTimer) clearInterval(scanTimer);
    if (tokioTimer) clearInterval(tokioTimer);
  });
</script>

<div class="min-h-screen bg-[#090a0d] text-slate-300 font-sans selection:bg-rust/80 selection:text-white pb-32">
  
  <!-- Subtle Header -->
  <header class="border-b border-white/[0.06] bg-[#090a0d]/90 backdrop-blur-md sticky top-0 z-40">
    <div class="max-w-6xl mx-auto px-6 h-16 flex items-center justify-between">
      <div class="flex items-center gap-6">
        <a href="/experiments" class="inline-flex items-center gap-2 text-xs font-mono text-slate-400 hover:text-white transition-colors no-underline">
          <ArrowLeft size={14} />
          <span>Experiments</span>
        </a>
        <div class="h-4 w-px bg-white/10"></div>
        <span class="text-xs font-mono text-slate-400">GhostShell Architecture Reference</span>
      </div>
      <div class="flex items-center gap-4 text-xs font-mono text-slate-400">
        <span class="hidden sm:inline">Edition: <strong class="text-slate-200">Rust 2024</strong></span>
        <span class="px-2 py-0.5 rounded bg-white/5 border border-white/10 text-slate-300">v0.2.0</span>
      </div>
    </div>
  </header>

  <div class="max-w-5xl mx-auto px-6 pt-16">
    
    <!-- Hero Title Section -->
    <div class="fade-in-up opacity-0 translate-y-3 mb-16 space-y-4">
      <div class="inline-block px-3 py-1 rounded-md bg-white/[0.04] border border-white/10 text-xs font-mono text-slate-400">
        Systems • Security • Network Daemons
      </div>
      <h1 class="text-4xl md:text-5xl font-bold text-white tracking-tight leading-tight">
        Advanced Rust Architecture & <br />
        <span class="text-slate-400 font-normal">eBPF/XDP Design Patterns</span>
      </h1>
      <p class="text-base md:text-lg text-slate-400 max-w-2xl font-light leading-relaxed">
        A deep-dive technical reference detailing the seven core engineering patterns behind high-throughput defensive daemons. Exploring zero-copy slice pipelines, RAII memory scrubbing, and exhaustive algebraic routing.
      </p>
    </div>

    <!-- Spacious Section Navigation Grid -->
    <div class="fade-in-up opacity-0 translate-y-3 grid grid-cols-2 sm:grid-cols-4 lg:grid-cols-7 gap-3 mb-16">
      {#each sections as sec, idx}
        <button
          on:click={() => activeSection = idx}
          class="text-left p-4 rounded-xl border transition-all duration-200 {activeSection === idx ? 'bg-[#13161c] border-white/20 text-white shadow-lg' : 'bg-[#0d0e12]/60 border-white/[0.06] text-slate-400 hover:border-white/10 hover:text-slate-200 hover:bg-[#0f1116]'}"
        >
          <div class="flex items-center justify-between mb-3 text-xs font-mono">
            <span class="{activeSection === idx ? 'text-rust' : 'text-slate-500'}">{sec.num}</span>
            <svelte:component this={sec.icon} size={16} class={activeSection === idx ? 'text-slate-200' : 'text-slate-500'} />
          </div>
          <div class="font-medium text-xs md:text-sm truncate mb-0.5">
            {sec.title}
          </div>
          <div class="text-[11px] text-slate-500 truncate">
            {sec.sub}
          </div>
        </button>
      {/each}
    </div>

    <!-- Main Content Area -->
    <main class="min-h-[500px]">
      
      <!-- ════════════════════════════════════════════════════════════════ -->
      <!-- 01: Zero-Copy Slice Pipeline                                     -->
      <!-- ════════════════════════════════════════════════════════════════ -->
      {#if activeSection === 0}
        <section class="space-y-12 animate-fadeIn">
          
          <div class="space-y-4 max-w-3xl">
            <div class="text-xs font-mono text-slate-400 flex items-center gap-2">
              <span>PATTERN 01</span>
              <span>•</span>
              <span>src/actions/mod.rs</span>
            </div>
            <h2 class="text-2xl md:text-3xl font-bold text-white tracking-tight">
              The eBPF/XDP-Style Zero-Copy Slice Pipeline (<code class="text-slate-200 font-mono text-xl">&[T]</code>)
            </h2>
            <p class="text-slate-400 leading-relaxed text-base">
              In high-speed network monitoring, heap allocation is a primary bottleneck. When a Linux kernel XDP hook intercepts a packet, it passes a lightweight pointer to the metadata (<code class="text-slate-300 font-mono">struct xdp_md *ctx</code>) without copying the payload to user space. GhostShell mirrors this architecture by passing immutable borrowed slices (<code class="text-slate-300 font-mono">&[Threat]</code>) through evaluation engines, eliminating heap cloning and garbage collection overhead entirely.
            </p>
          </div>

          <!-- Interactive Benchmark Box -->
          <div class="p-8 rounded-2xl bg-[#0d0e12] border border-white/[0.08] space-y-8">
            <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 pb-6 border-b border-white/[0.06]">
              <div>
                <h3 class="font-medium text-white text-base">Evaluation Throughput Bench</h3>
                <p class="text-xs text-slate-400 mt-1">Compare memory churn and latency across allocation strategies.</p>
              </div>
              <div class="flex items-center gap-3">
                <div class="bg-black/40 p-1 rounded-lg border border-white/[0.08] flex text-xs font-mono">
                  <button
                    on:click={() => { simMode = "vec"; resetSim(); }}
                    class="px-3 py-1.5 rounded transition-all {simMode === 'vec' ? 'bg-white/10 text-white' : 'text-slate-400 hover:text-slate-200'}"
                  >
                    Vec&lt;Threat&gt; (Heap Clone)
                  </button>
                  <button
                    on:click={() => { simMode = "slice"; resetSim(); }}
                    class="px-3 py-1.5 rounded transition-all {simMode === 'slice' ? 'bg-white/10 text-white' : 'text-slate-400 hover:text-slate-200'}"
                  >
                    &[Threat] (Zero-Copy)
                  </button>
                </div>
                <button
                  on:click={toggleScan}
                  class="px-4 py-1.5 rounded-lg font-mono text-xs font-medium flex items-center gap-2 transition-all {isScanning ? 'bg-white/10 text-white border border-white/20' : 'bg-rust text-white hover:bg-rust/90'}"
                >
                  {#if isScanning}<Pause size={14} /> <span>Pause</span>{:else}<Play size={14} /> <span>Run Bench</span>{/if}
                </button>
              </div>
            </div>

            <!-- Metrics Grid -->
            <div class="grid grid-cols-1 sm:grid-cols-3 gap-6">
              <div class="p-5 rounded-xl bg-black/40 border border-white/[0.05]">
                <span class="text-xs font-mono text-slate-500 block mb-1">HEAP ALLOCATIONS</span>
                <div class="text-2xl font-mono font-medium {simMode === 'vec' ? 'text-red-400' : 'text-slate-200'}">
                  {allocCount.toLocaleString()} <span class="text-xs font-normal text-slate-500">allocs</span>
                </div>
                <div class="text-xs text-slate-400 mt-2">
                  {simMode === 'vec' ? 'High malloc/free pressure' : '0 heap allocations (stack pointer)'}
                </div>
              </div>

              <div class="p-5 rounded-xl bg-black/40 border border-white/[0.05]">
                <span class="text-xs font-mono text-slate-500 block mb-1">MEMORY FRAGMENTATION</span>
                <div class="text-2xl font-mono font-medium {simMode === 'vec' ? 'text-orange-400' : 'text-slate-200'}">
                  {heapMemoryKb.toLocaleString()} <span class="text-xs font-normal text-slate-500">KB churn</span>
                </div>
                <div class="text-xs text-slate-400 mt-2">
                  {simMode === 'vec' ? 'Deallocation delay active' : 'Zero memory footprint growth'}
                </div>
              </div>

              <div class="p-5 rounded-xl bg-black/40 border border-white/[0.05]">
                <span class="text-xs font-mono text-slate-500 block mb-1">EVALUATION SPEED</span>
                <div class="text-2xl font-mono font-medium text-white">
                  {opsPerSec.toLocaleString()} <span class="text-xs font-normal text-slate-500">ops/s</span>
                </div>
                <div class="text-xs text-slate-400 mt-2">
                  {simMode === 'vec' ? 'Constrained by memory bus' : 'L1 cache contiguous velocity'}
                </div>
              </div>
            </div>

            <!-- Log output -->
            {#if streamLog.length > 0}
              <div class="space-y-2 pt-2">
                <span class="text-xs font-mono text-slate-500 block">EXECUTION LOG</span>
                <div class="space-y-1 font-mono text-xs bg-black/40 p-4 rounded-xl border border-white/[0.05]">
                  {#each streamLog as item (item.id)}
                    <div class="flex justify-between text-slate-300 py-0.5">
                      <span>{item.text}</span>
                      <span class={item.mode === 'vec' ? 'text-red-400' : 'text-emerald-400'}>{item.latency}</span>
                    </div>
                  {/each}
                </div>
              </div>
            {/if}
          </div>

          <!-- Code blocks -->
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div class="p-6 rounded-xl bg-[#0d0e12] border border-white/[0.06] font-mono text-xs space-y-3">
              <div class="text-slate-400 flex justify-between border-b border-white/[0.06] pb-3">
                <span>Traditional Allocation</span>
                <span class="text-red-400 font-medium">Vec&lt;T&gt;</span>
              </div>
              <pre class="text-slate-300 leading-relaxed overflow-x-auto"><code><span class="text-slate-500">// Moves ownership and allocates on heap</span>
pub fn evaluate(&mut self, threats: <span class="text-red-400">Vec&lt;Threat&gt;</span>) -> Vec&lt;Action&gt; &#123;
    let mut actions = Vec::new();
    for threat in threats &#123;
        if threat.confidence > 0.8 &#123;
            actions.push(Action::Neutralize(Eliminate));
        &#125;
    &#125;
    actions
&#125;</code></pre>
            </div>

            <div class="p-6 rounded-xl bg-[#0d0e12] border border-white/[0.12] font-mono text-xs space-y-3">
              <div class="text-slate-300 flex justify-between border-b border-white/[0.06] pb-3 font-medium">
                <span>Zero-Copy Slice Pipeline</span>
                <span class="text-emerald-400 font-medium">&[T]</span>
              </div>
              <pre class="text-slate-300 leading-relaxed overflow-x-auto"><code><span class="text-slate-500">// Borrowed contiguous slice (xdp_md *ctx equivalent)</span>
pub fn evaluate(&mut self, threats: <span class="text-emerald-400">&[Threat]</span>) -> Vec&lt;Action&gt; &#123;
    let mut actions = Vec::with_capacity(threats.len());
    for threat in threats &#123; <span class="text-slate-500">// Direct pointer inspection</span>
        if threat.confidence > 0.8 &#123;
            actions.push(Action::Neutralize(Eliminate));
        &#125;
    &#125;
    actions
&#125;</code></pre>
            </div>
          </div>

        </section>
      {/if}

      <!-- ════════════════════════════════════════════════════════════════ -->
      <!-- 02: Active Zeroization                                           -->
      <!-- ════════════════════════════════════════════════════════════════ -->
      {#if activeSection === 1}
        <section class="space-y-12 animate-fadeIn">
          
          <div class="space-y-4 max-w-3xl">
            <div class="text-xs font-mono text-slate-400 flex items-center gap-2">
              <span>PATTERN 02</span>
              <span>•</span>
              <span>src/core/crypto.rs</span>
            </div>
            <h2 class="text-2xl md:text-3xl font-bold text-white tracking-tight">
              Active Memory Zeroization & Cryptographic Hygiene
            </h2>
            <p class="text-slate-400 leading-relaxed text-base">
              Sensitive cryptographic keys residing in RAM remain readable in memory dumps even after variables go out of scope. When developers attempt manual zeroing before dropping a buffer (<code class="text-slate-300 font-mono">buffer.fill(0)</code>), LLVM compiler optimizations frequently eliminate the loop as a dead-store optimization. By implementing <code class="text-slate-300 font-mono">zeroize::Zeroize</code> within Rust's <code class="text-slate-300 font-mono">Drop</code> trait, we enforce volatile memory writes and compiler fences that guarantee physical RAM scrubbing.
            </p>
          </div>

          <!-- Memory Scrubbing Sandbox -->
          <div class="p-8 rounded-2xl bg-[#0d0e12] border border-white/[0.08] space-y-8">
            <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 pb-6 border-b border-white/[0.06]">
              <div>
                <h3 class="font-medium text-white text-base">RAM Page Inspection Bench</h3>
                <p class="text-xs text-slate-400 mt-1">Simulate memory deallocation across physical address range 0x7F00–0x7F07.</p>
              </div>
              <div class="flex gap-2.5 text-xs font-mono">
                <button
                  on:click={resetMemory}
                  class="px-3.5 py-2 rounded-lg bg-white/5 hover:bg-white/10 border border-white/10 text-slate-300 transition-colors"
                >
                  Allocate Secrets
                </button>
                <button
                  on:click={dropWithoutZeroize}
                  class="px-3.5 py-2 rounded-lg bg-red-500/10 hover:bg-red-500/20 border border-red-500/20 text-red-400 transition-colors"
                >
                  Drop (Standard)
                </button>
                <button
                  on:click={dropWithZeroize}
                  class="px-3.5 py-2 rounded-lg bg-emerald-500/15 hover:bg-emerald-500/25 border border-emerald-500/30 text-emerald-300 transition-colors font-medium"
                >
                  Drop (with zeroize::Zeroize)
                </button>
              </div>
            </div>

            <!-- Hex Cells Grid -->
            <div class="grid grid-cols-2 sm:grid-cols-4 md:grid-cols-8 gap-3">
              {#each memCells as cell}
                <div class="p-4 rounded-xl border text-center transition-all duration-200 {memState === 'zeroized' ? 'bg-emerald-950/10 border-emerald-500/30' : (memState === 'dropped_dead' ? 'bg-red-950/10 border-red-500/30' : 'bg-black/40 border-white/[0.06]')}">
                  <div class="text-[11px] font-mono text-slate-500 mb-1.5">{cell.addr}</div>
                  <div class="text-2xl font-mono font-bold tracking-wide {memState === 'zeroized' ? 'text-emerald-400' : (memState === 'dropped_dead' ? 'text-red-400' : 'text-white')}">
                    {cell.val}
                  </div>
                  <div class="text-[11px] font-mono text-slate-400 mt-1.5 truncate">
                    {memState === 'zeroized' ? '0x00_WIPED' : cell.label}
                  </div>
                </div>
              {/each}
            </div>

            <!-- Status banner -->
            <div class="p-4 rounded-xl border text-xs font-mono leading-relaxed {memState === 'zeroized' ? 'bg-emerald-500/10 border-emerald-500/20 text-emerald-300' : (memState === 'dropped_dead' ? 'bg-red-500/10 border-red-500/20 text-red-300' : 'bg-white/[0.02] border-white/[0.06] text-slate-300')}">
              {#if memState === 'active'}
                Status: Cryptographic keys allocated in plaintext RAM. Variable currently in scope.
              {:else if memState === 'dropped_dead'}
                Warning: Variable dropped from scope, but standard memory cleanup was eliminated by LLVM dead-store optimization. Plaintext bytes remain vulnerable in RAM.
              {:else}
                Verified: Volatile memory writes and atomic compiler fences forced physical RAM overwrites to 0x00 before deallocation completed.
              {/if}
            </div>
          </div>

          <!-- Code Snippet -->
          <div class="p-6 rounded-xl bg-[#0d0e12] border border-white/[0.06] font-mono text-xs space-y-3">
            <div class="text-slate-400 flex justify-between border-b border-white/[0.06] pb-3">
              <span>src/core/crypto.rs</span>
              <span class="text-slate-300 font-medium">RAII Drop Implementation</span>
            </div>
            <pre class="text-slate-300 leading-relaxed overflow-x-auto"><code>use zeroize::Zeroize;

#[derive(Debug, Clone)]
pub struct SecureMemory &#123;
    buffer: Vec&lt;u8&gt;,
&#125;

impl Drop for SecureMemory &#123;
    fn drop(&mut self) &#123;
        <span class="text-slate-500">// Volatile write barriers defeat dead-store elimination</span>
        <span class="text-slate-500">// Executed deterministically when scope exits or during panic unwind</span>
        self.buffer.zeroize();
    &#125;
&#125;</code></pre>
          </div>

        </section>
      {/if}

      <!-- ════════════════════════════════════════════════════════════════ -->
      <!-- 03: Exhaustive Verdicts                                          -->
      <!-- ════════════════════════════════════════════════════════════════ -->
      {#if activeSection === 2}
        <section class="space-y-12 animate-fadeIn">
          
          <div class="space-y-4 max-w-3xl">
            <div class="text-xs font-mono text-slate-400 flex items-center gap-2">
              <span>PATTERN 03</span>
              <span>•</span>
              <span>src/actions/mod.rs</span>
            </div>
            <h2 class="text-2xl md:text-3xl font-bold text-white tracking-tight">
              Verdict-Driven Countermeasures via Exhaustive ADTs
            </h2>
            <p class="text-slate-400 leading-relaxed text-base">
              In Linux XDP networking, every packet evaluation must resolve to an explicit kernel verdict (<code class="text-slate-300 font-mono">XDP_PASS</code>, <code class="text-slate-300 font-mono">XDP_DROP</code>, <code class="text-slate-300 font-mono">XDP_TX</code>). In GhostShell, threat evaluation terminates in an <code class="text-slate-300 font-mono">Action</code> Algebraic Data Type (ADT). Rust's exhaustive <code class="text-slate-300 font-mono">match</code> checking guarantees at compile time that every possible threat variant has an explicit routing handler.
            </p>
          </div>

          <!-- Verdict Routing Box -->
          <div class="p-8 rounded-2xl bg-[#0d0e12] border border-white/[0.08] space-y-8">
            <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 pb-6 border-b border-white/[0.06]">
              <div>
                <h3 class="font-medium text-white text-base">Verdict Routing Engine</h3>
                <p class="text-xs text-slate-400 mt-1">Select threat signatures to test exhaustive enum matching.</p>
              </div>
              <div class="flex flex-wrap gap-2 text-xs font-mono">
                <button
                  on:click={() => injectThreat("Critical", 0.96, "198.51.100.24", "Kernel Rootkit Attempt")}
                  class="px-3 py-1.5 rounded bg-red-500/10 hover:bg-red-500/20 border border-red-500/20 text-red-300 transition-colors"
                >
                  Critical Attack
                </button>
                <button
                  on:click={() => injectThreat("High", 0.76, "203.0.113.88", "SYN Flood Attack")}
                  class="px-3 py-1.5 rounded bg-orange-500/10 hover:bg-orange-500/20 border border-orange-500/20 text-orange-300 transition-colors"
                >
                  SYN Flood
                </button>
                <button
                  on:click={() => injectThreat("Medium", 0.52, "192.0.2.10", "SSH Brute Force")}
                  class="px-3 py-1.5 rounded bg-yellow-500/10 hover:bg-yellow-500/20 border border-yellow-500/20 text-yellow-300 transition-colors"
                >
                  SSH Brute Force
                </button>
                <button
                  on:click={() => injectThreat("Low", 0.12, "10.0.0.4", "Benign ICMP Echo")}
                  class="px-3 py-1.5 rounded bg-white/5 hover:bg-white/10 border border-white/10 text-slate-300 transition-colors"
                >
                  Benign Traffic
                </button>
              </div>
            </div>

            <!-- Routing Display -->
            <div class="grid grid-cols-1 md:grid-cols-2 gap-6 font-mono text-xs">
              <div class="p-5 rounded-xl bg-black/40 border border-white/[0.05] space-y-3">
                <span class="text-slate-500 block">THREAT METADATA</span>
                <div class="space-y-1.5">
                  <div class="flex justify-between"><span class="text-slate-400">Signature:</span><span class="text-white font-medium">{selectedThreat.type}</span></div>
                  <div class="flex justify-between"><span class="text-slate-400">Severity:</span><span class="text-slate-200">{selectedThreat.sev}</span></div>
                  <div class="flex justify-between"><span class="text-slate-400">Confidence:</span><span class="text-slate-200">{(selectedThreat.conf * 100).toFixed(0)}%</span></div>
                  <div class="flex justify-between"><span class="text-slate-400">Source IP:</span><span class="text-slate-300">{selectedThreat.ip}</span></div>
                </div>
              </div>

              <div class="p-5 rounded-xl bg-black/40 border border-white/[0.05] space-y-3 flex flex-col justify-between">
                <div>
                  <span class="text-slate-500 block mb-2">RESOLVED ADT VERDICT</span>
                  <div class="p-3.5 rounded-lg border font-medium text-sm {verdictStyle}">
                    {currentVerdict}
                  </div>
                </div>
                <div class="text-[11px] text-slate-500 pt-2 border-t border-white/[0.05]">
                  Compile-time guarantee: adding new enum variants without handling them breaks build compilation.
                </div>
              </div>
            </div>

            <!-- Recent Log -->
            <div class="space-y-2 pt-2 font-mono text-xs">
              <span class="text-slate-500 block">RECENT VERDICT ROUTING</span>
              <div class="grid grid-cols-1 sm:grid-cols-3 gap-3">
                {#each historyLog as hist}
                  <div class="p-3 rounded-lg bg-black/40 border border-white/[0.05] flex justify-between items-center">
                    <div>
                      <span class="text-slate-200 font-medium">{hist.id}</span>
                      <span class="text-[11px] text-slate-500 block">{hist.sev} Severity</span>
                    </div>
                    <div class="text-right">
                      <span class="text-slate-300 text-[11px]">{hist.verdict}</span>
                      <span class="text-[10px] text-slate-500 block mt-0.5">{hist.time}</span>
                    </div>
                  </div>
                {/each}
              </div>
            </div>
          </div>

          <!-- Code Snippet -->
          <div class="p-6 rounded-xl bg-[#0d0e12] border border-white/[0.06] font-mono text-xs space-y-3">
            <div class="text-slate-400 flex justify-between border-b border-white/[0.06] pb-3">
              <span>src/actions/mod.rs</span>
              <span class="text-slate-300 font-medium">Exhaustive Match Routing</span>
            </div>
            <pre class="text-slate-300 leading-relaxed overflow-x-auto"><code>#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action &#123;
    Neutralize(NeutralizationMethod), <span class="text-slate-500">// Equivalent to XDP_DROP</span>
    Counter,
    Probe,                            <span class="text-slate-500">// Equivalent to XDP_TX</span>
    Deceive,
    Ignore,                           <span class="text-slate-500">// Equivalent to XDP_PASS</span>
&#125;

<span class="text-slate-500">// Exhaustive routing block</span>
match action &#123;
    Action::Neutralize(method) =&gt; self.neutralize_engine.execute(threat, method),
    Action::Counter            =&gt; self.counter_engine.deploy(threat),
    Action::Probe              =&gt; self.probe_engine.scan(threat),
    Action::Deceive            =&gt; self.deceive_engine.mislead(threat),
    Action::Ignore             =&gt; Ok(ActionResult::ignored()),
&#125;</code></pre>
          </div>

        </section>
      {/if}

      <!-- ════════════════════════════════════════════════════════════════ -->
      <!-- 04: Resilient Configuration                                      -->
      <!-- ════════════════════════════════════════════════════════════════ -->
      {#if activeSection === 3}
        <section class="space-y-12 animate-fadeIn">
          
          <div class="space-y-4 max-w-3xl">
            <div class="text-xs font-mono text-slate-400 flex items-center gap-2">
              <span>PATTERN 04</span>
              <span>•</span>
              <span>src/core/config.rs</span>
            </div>
            <h2 class="text-2xl md:text-3xl font-bold text-white tracking-tight">
              Resilient Configuration with Serde Default Fallbacks
            </h2>
            <p class="text-slate-400 leading-relaxed text-base">
              When daemons evolve, new configuration keys are added over time. If an existing server runs an older config file lacking newly introduced fields, standard parsers fail and terminate startup. By annotating structs with <code class="text-slate-300 font-mono">#[serde(default)]</code> and implementing the <code class="text-slate-300 font-mono">Default</code> trait, Serde automatically injects safe fallbacks for missing schema fields, enabling zero-downtime upgrades.
            </p>
          </div>

          <!-- Config Sandbox -->
          <div class="p-8 rounded-2xl bg-[#0d0e12] border border-white/[0.08] space-y-8">
            <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 pb-6 border-b border-white/[0.06]">
              <div>
                <h3 class="font-medium text-white text-base">Schema Upgrade Sandbox</h3>
                <p class="text-xs text-slate-400 mt-1">Select configuration payloads to observe default value injection.</p>
              </div>
              <div class="flex gap-2 text-xs font-mono">
                <button
                  on:click={() => configScenario = "missing_agent"}
                  class="px-3.5 py-1.5 rounded transition-all {configScenario === 'missing_agent' ? 'bg-white/10 text-white' : 'bg-white/5 text-slate-400 hover:text-slate-200'}"
                >
                  Legacy Config (v0.1)
                </button>
                <button
                  on:click={() => configScenario = "complete"}
                  class="px-3.5 py-1.5 rounded transition-all {configScenario === 'complete' ? 'bg-white/10 text-white' : 'bg-white/5 text-slate-400 hover:text-slate-200'}"
                >
                  Complete Config (v0.2)
                </button>
                <button
                  on:click={() => configScenario = "corrupt"}
                  class="px-3.5 py-1.5 rounded transition-all {configScenario === 'corrupt' ? 'bg-white/10 text-white' : 'bg-white/5 text-slate-400 hover:text-slate-200'}"
                >
                  Malformed Syntax
                </button>
              </div>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-6 font-mono text-xs">
              <div class="p-5 rounded-xl bg-black/40 border border-white/[0.05] space-y-3">
                <span class="text-slate-500 block">INPUT YAML FILE</span>
                <pre class="text-slate-300 overflow-x-auto leading-relaxed min-h-[120px]"><code>{#if configScenario === 'missing_agent'}daemon_name: "ghost-worker-node-44"
<span class="text-slate-500"># Note: 'agent' block is omitted in v0.1 schema.</span>
<span class="text-slate-500"># Serde will fall back to Default::default().</span>{:else if configScenario === 'complete'}daemon_name: "ghost-worker-node-44"
agent:
  max_concurrent_ops: 25
  scan_interval_ms: 500
  strict_mode: false{:else}daemon_name: "ghost-worker-node-44"
agent: [INVALID_YAML_SYNTAX
  max_concurrent_ops: "NOT_AN_INT"{/if}</code></pre>
              </div>

              <div class="p-5 rounded-xl bg-black/40 border {configScenario === 'corrupt' ? 'border-red-500/20' : 'border-white/[0.05]'} space-y-3">
                <span class="text-slate-500 block">DESERIALIZED RUST STRUCT</span>
                <pre class="overflow-x-auto leading-relaxed min-h-[120px] {configScenario === 'corrupt' ? 'text-red-300' : 'text-slate-200'}"><code>{#if configScenario === 'missing_agent'}Config &#123;
    daemon_name: "ghost-worker-node-44",
    agent: AgentConfig &#123;
        max_concurrent_ops: 10, <span class="text-slate-500">// Injected default</span>
        scan_interval_ms: 1000, <span class="text-slate-500">// Injected default</span>
        strict_mode: true       <span class="text-slate-500">// Injected default</span>
    &#125;
&#125;{:else if configScenario === 'complete'}Config &#123;
    daemon_name: "ghost-worker-node-44",
    agent: AgentConfig &#123;
        max_concurrent_ops: 25,
        scan_interval_ms: 500,
        strict_mode: false
    &#125;
&#125;{:else}Err(serde_yaml::Error(
    "invalid type: string \"NOT_AN_INT\", expected usize"
))
<span class="text-slate-500">// Error trapped cleanly without panic</span>{/if}</code></pre>
              </div>
            </div>
          </div>

          <!-- Code Snippet -->
          <div class="p-6 rounded-xl bg-[#0d0e12] border border-white/[0.06] font-mono text-xs space-y-3">
            <div class="text-slate-400 flex justify-between border-b border-white/[0.06] pb-3">
              <span>src/core/config.rs</span>
              <span class="text-slate-300 font-medium">Serde Macro Annotation</span>
            </div>
            <pre class="text-slate-300 leading-relaxed overflow-x-auto"><code>use serde::&#123;Deserialize, Serialize&#125;;

#[derive(Debug, Clone, Serialize, Deserialize)]
<span class="text-slate-300">#[serde(default)] // Instructs Serde to use Default::default() for missing fields</span>
pub struct AgentConfig &#123;
    pub max_concurrent_ops: usize,
    pub scan_interval_ms: u64,
    pub strict_mode: bool,
&#125;

impl Default for AgentConfig &#123;
    fn default() -&gt; Self &#123;
        Self &#123; max_concurrent_ops: 10, scan_interval_ms: 1000, strict_mode: true &#125;
    &#125;
&#125;</code></pre>
          </div>

        </section>
      {/if}

      <!-- ════════════════════════════════════════════════════════════════ -->
      <!-- 05: Async Actor Architecture                                     -->
      <!-- ════════════════════════════════════════════════════════════════ -->
      {#if activeSection === 4}
        <section class="space-y-12 animate-fadeIn">
          
          <div class="space-y-4 max-w-3xl">
            <div class="text-xs font-mono text-slate-400 flex items-center gap-2">
              <span>PATTERN 05</span>
              <span>•</span>
              <span>src/agent/mod.rs</span>
            </div>
            <h2 class="text-2xl md:text-3xl font-bold text-white tracking-tight">
              Asynchronous Actor Architecture (<code class="text-slate-200 font-mono text-xl">tokio</code>)
            </h2>
            <p class="text-slate-400 leading-relaxed text-base">
              GhostShell structures internal monitoring engines as decoupled concurrent actors. When an asynchronous loop invokes <code class="text-slate-300 font-mono">.await</code>, control is yielded back to the Tokio runtime without blocking the underlying OS thread. Tokio's work-stealing scheduler dynamically distributes pending tasks across CPU worker cores, enabling concurrent monitoring of thousands of network connections and system events.
            </p>
          </div>

          <!-- Worker Core Simulator -->
          <div class="p-8 rounded-2xl bg-[#0d0e12] border border-white/[0.08] space-y-8">
            <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 pb-6 border-b border-white/[0.06]">
              <div>
                <h3 class="font-medium text-white text-base">Work-Stealing Runtime Bench</h3>
                <p class="text-xs text-slate-400 mt-1">Simulate task distribution across 4 asynchronous CPU worker cores.</p>
              </div>
              <button
                on:click={toggleTokioSim}
                class="px-4 py-2 rounded-lg font-mono text-xs font-medium flex items-center gap-2 transition-all {isTokioRunning ? 'bg-white/10 text-white border border-white/20' : 'bg-rust text-white hover:bg-rust/90'}"
              >
                {#if isTokioRunning}<Pause size={14} /> <span>Pause Runtime</span>{:else}<Play size={14} /> <span>Run Simulation</span>{/if}
              </button>
            </div>

            <!-- Cores Grid -->
            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 font-mono text-xs">
              {#each workerCores as core}
                <div class="p-5 rounded-xl bg-black/40 border border-white/[0.05] space-y-4">
                  <div class="flex justify-between items-center text-slate-300">
                    <span class="font-medium">{core.name}</span>
                    <span class="text-[11px] text-slate-500">{core.status}</span>
                  </div>
                  <div class="space-y-1.5">
                    <div class="flex justify-between text-[11px] text-slate-400">
                      <span>Core Load</span>
                      <span class="text-slate-200 font-medium">{core.load}%</span>
                    </div>
                    <div class="w-full bg-white/5 h-1.5 rounded-full overflow-hidden">
                      <div class="bg-slate-300 h-full transition-all duration-300" style="width: {core.load}%"></div>
                    </div>
                  </div>
                  <div class="flex justify-between text-[11px] text-slate-400 pt-2 border-t border-white/[0.05]">
                    <span>Tasks Queued:</span>
                    <span class="text-slate-200">{core.tasks}</span>
                  </div>
                </div>
              {/each}
            </div>

            <div class="p-4 rounded-xl bg-black/40 border border-white/[0.05] grid grid-cols-1 sm:grid-cols-3 gap-4 font-mono text-xs text-center">
              <div>
                <span class="text-slate-500 block text-[11px]">COOPERATIVE YIELD RATE</span>
                <span class="text-slate-200 font-medium text-sm">{yieldRate.toLocaleString()} <span class="text-xs text-slate-500 font-normal">yields/sec</span></span>
              </div>
              <div>
                <span class="text-slate-500 block text-[11px]">OS THREAD BLOCKING</span>
                <span class="text-slate-200 font-medium text-sm">0.00 ms <span class="text-xs text-slate-500 font-normal">(Non-blocking)</span></span>
              </div>
              <div>
                <span class="text-slate-500 block text-[11px]">SCHEDULER BALANCE</span>
                <span class="text-slate-200 font-medium text-sm">99.8% <span class="text-xs text-slate-500 font-normal">efficiency</span></span>
              </div>
            </div>
          </div>

          <!-- Code Snippet -->
          <div class="p-6 rounded-xl bg-[#0d0e12] border border-white/[0.06] font-mono text-xs space-y-3">
            <div class="text-slate-400 flex justify-between border-b border-white/[0.06] pb-3">
              <span>src/agent/mod.rs</span>
              <span class="text-slate-300 font-medium">Cooperative Futures Loop</span>
            </div>
            <pre class="text-slate-300 leading-relaxed overflow-x-auto"><code>impl GhostAgent &#123;
    pub async fn run(&mut self) -&gt; Result&lt;()&gt; &#123;
        <span class="text-slate-500">// Non-blocking monitoring loop yielding to Tokio work-stealing runtime</span>
        loop &#123;
            <span class="text-slate-500">// .await yields execution while network sensors scan I/O interfaces</span>
            let threats = self.perception.scan().await?;
            if !threats.is_empty() &#123;
                self.actions.execute_all(&threats).await?;
            &#125;
            tokio::time::sleep(Duration::from_secs(5)).await;
        &#125;
    &#125;
&#125;</code></pre>
          </div>

        </section>
      {/if}

      <!-- ════════════════════════════════════════════════════════════════ -->
      <!-- 06: Structured Telemetry                                         -->
      <!-- ════════════════════════════════════════════════════════════════ -->
      {#if activeSection === 5}
        <section class="space-y-12 animate-fadeIn">
          
          <div class="space-y-4 max-w-3xl">
            <div class="text-xs font-mono text-slate-400 flex items-center gap-2">
              <span>PATTERN 06</span>
              <span>•</span>
              <span>src/core/logger.rs</span>
            </div>
            <h2 class="text-2xl md:text-3xl font-bold text-white tracking-tight">
              Enterprise Structured Telemetry (<code class="text-slate-200 font-mono text-xl">tracing</code>)
            </h2>
            <p class="text-slate-400 leading-relaxed text-base">
              Synchronous logging via <code class="text-slate-300 font-mono">println!</code> locks standard output on every call, creating severe mutex bottlenecks during high-frequency network events. Rust's <code class="text-slate-300 font-mono">tracing</code> framework records events as structured key-value pairs (<code class="text-slate-300 font-mono">threat_id = %threat.id</code>), allowing formatters to emit machine-readable JSON logs for SIEM pipelines with zero-cost runtime filtering via <code class="text-slate-300 font-mono">EnvFilter</code>.
            </p>
          </div>

          <!-- Telemetry Switcher Box -->
          <div class="p-8 rounded-2xl bg-[#0d0e12] border border-white/[0.08] space-y-8">
            <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 pb-6 border-b border-white/[0.06]">
              <div>
                <h3 class="font-medium text-white text-base">Telemetry Formatting Bench</h3>
                <p class="text-xs text-slate-400 mt-1">Switch output formatters to compare log ingestion formats.</p>
              </div>
              <div class="flex gap-2 text-xs font-mono">
                <button
                  on:click={() => logMode = "println"}
                  class="px-3 py-1.5 rounded transition-all {logMode === 'println' ? 'bg-white/10 text-white' : 'bg-white/5 text-slate-400 hover:text-slate-200'}"
                >
                  Raw Text (println!)
                </button>
                <button
                  on:click={() => logMode = "tracing_term"}
                  class="px-3 py-1.5 rounded transition-all {logMode === 'tracing_term' ? 'bg-white/10 text-white' : 'bg-white/5 text-slate-400 hover:text-slate-200'}"
                >
                  Terminal Formatter
                </button>
                <button
                  on:click={() => logMode = "tracing_json"}
                  class="px-3 py-1.5 rounded transition-all {logMode === 'tracing_json' ? 'bg-white/10 text-white' : 'bg-white/5 text-slate-400 hover:text-slate-200'}"
                >
                  JSON Line (SIEM)
                </button>
              </div>
            </div>

            <!-- Log Stream Window -->
            <div class="p-5 rounded-xl bg-black/40 border border-white/[0.05] font-mono text-xs space-y-3 min-h-[160px]">
              {#if logMode === 'println'}
                <div class="text-slate-300">Neutralizing active threat THR-001 with Eliminate</div>
                <div class="text-slate-300">Evaluating verdict match tree for THR-001</div>
                <div class="text-slate-300">High concurrent scan pressure detected</div>
                <div class="text-[11px] text-slate-500 pt-3 border-t border-white/[0.05]">
                  Note: Synchronous stdout mutex locking delays thread execution during packet surges. Unparseable as structured JSON.
                </div>
              {:else if logMode === 'tracing_term'}
                {#each sampleLogs as lg}
                  <div class="flex flex-wrap items-start gap-3 py-1 text-slate-300 border-b border-white/[0.03]">
                    <span class="text-slate-500 shrink-0">{lg.time}Z</span>
                    <span class="px-1.5 py-0.5 rounded text-[10px] bg-white/10 text-slate-200 shrink-0">{lg.level}</span>
                    <span class="text-slate-400 shrink-0">{lg.target}:</span>
                    <span>{lg.msg}</span>
                    {#if lg.threat_id}<span class="text-slate-400">threat_id={lg.threat_id}</span>{/if}
                  </div>
                {/each}
              {:else}
                {#each sampleLogs as lg}
                  <div class="text-slate-300 py-1.5 border-b border-white/[0.03] overflow-x-auto">
                    &#123;"timestamp":"2026-07-01T{lg.time}Z","level":"{lg.level}","target":"{lg.target}","fields":&#123;"message":"{lg.msg}"{#if lg.threat_id},"threat_id":"{lg.threat_id}"{/if}{#if lg.method},"method":"{lg.method}"{/if}&#125;&#125;
                  </div>
                {/each}
                <div class="text-[11px] text-slate-500 pt-3 border-t border-white/[0.05]">
                  Structured JSON output ready for direct ingestion by Elasticsearch or Splunk. Zero synchronous mutex locking.
                </div>
              {/if}
            </div>
          </div>

          <!-- Code Snippet -->
          <div class="p-6 rounded-xl bg-[#0d0e12] border border-white/[0.06] font-mono text-xs space-y-3">
            <div class="text-slate-400 flex justify-between border-b border-white/[0.06] pb-3">
              <span>src/core/logger.rs</span>
              <span class="text-slate-300 font-medium">Subscriber Configuration</span>
            </div>
            <pre class="text-slate-300 leading-relaxed overflow-x-auto"><code>use tracing::&#123;info, warn, error, Level&#125;;
use tracing_subscriber::&#123;fmt, prelude::*, EnvFilter&#125;;

pub fn init_with_config() &#123;
    let filter = EnvFilter::from_default_env().add_directive(Level::INFO.into());

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().with_target(false).with_timer(fmt::time::UtcTime::rfc_3339()))
        .init();
&#125;

<span class="text-slate-500">// Structured key-value logging across modules:</span>
info!(threat_id = %threat.id, confidence = threat.confidence, "Neutralizing active threat");</code></pre>
          </div>

        </section>
      {/if}

      <!-- ════════════════════════════════════════════════════════════════ -->
      <!-- 07: Dual-Layer Error Propagation                                 -->
      <!-- ════════════════════════════════════════════════════════════════ -->
      {#if activeSection === 6}
        <section class="space-y-12 animate-fadeIn">
          
          <div class="space-y-4 max-w-3xl">
            <div class="text-xs font-mono text-slate-400 flex items-center gap-2">
              <span>PATTERN 07</span>
              <span>•</span>
              <span>src/core/mod.rs</span>
            </div>
            <h2 class="text-2xl md:text-3xl font-bold text-white tracking-tight">
              Dual-Layer Error Propagation Architecture
            </h2>
            <p class="text-slate-400 leading-relaxed text-base">
              GhostShell combines two distinct error handling philosophies: <code class="text-slate-300 font-mono">thiserror</code> inside library modules generates precise domain error enums at compile time without runtime overhead. <code class="text-slate-300 font-mono">anyhow</code> is used in application entry points and orchestration loops to capture backtraces and attach chronological causality context via <code class="text-slate-300 font-mono">.context(...)</code> using the ergonomic <code class="text-slate-300 font-mono">?</code> operator.
            </p>
          </div>

          <!-- Error Trajectory Box -->
          <div class="p-8 rounded-2xl bg-[#0d0e12] border border-white/[0.08] space-y-8">
            <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 pb-6 border-b border-white/[0.06]">
              <div>
                <h3 class="font-medium text-white text-base">Error Causality Chain Explorer</h3>
                <p class="text-xs text-slate-400 mt-1">Select failure modes to inspect unified error unwinding.</p>
              </div>
              <div class="flex gap-2 text-xs font-mono">
                <button
                  on:click={() => errorScenario = "tamper"}
                  class="px-3 py-1.5 rounded transition-all {errorScenario === 'tamper' ? 'bg-white/10 text-white' : 'bg-white/5 text-slate-400 hover:text-slate-200'}"
                >
                  Crypto Tag Mismatch
                </button>
                <button
                  on:click={() => errorScenario = "missing_file"}
                  class="px-3 py-1.5 rounded transition-all {errorScenario === 'missing_file' ? 'bg-white/10 text-white' : 'bg-white/5 text-slate-400 hover:text-slate-200'}"
                >
                  Config I/O Error
                </button>
                <button
                  on:click={() => errorScenario = "socket"}
                  class="px-3 py-1.5 rounded transition-all {errorScenario === 'socket' ? 'bg-white/10 text-white' : 'bg-white/5 text-slate-400 hover:text-slate-200'}"
                >
                  Stream Timeout
                </button>
              </div>
            </div>

            <!-- Causality Trace Display -->
            <div class="p-6 rounded-xl bg-black/40 border border-white/[0.05] font-mono text-xs space-y-4">
              <div class="text-slate-400 flex justify-between border-b border-white/[0.05] pb-2">
                <span>CAUSALITY TRACE</span>
                <span class="text-[11px] text-slate-500">Bubbled via `?` operator</span>
              </div>

              <div class="space-y-4 pl-3 border-l border-white/20">
                {#if errorScenario === 'tamper'}
                  <div class="space-y-1">
                    <span class="text-[10px] text-slate-500 block">LEVEL 0 • APPLICATION CONTEXT (anyhow::Context)</span>
                    <div class="text-white font-medium">Error: Cryptographic integrity check failed while loading daemon config</div>
                    <div class="text-[11px] text-slate-500">at src/main.rs:214</div>
                  </div>
                  <div class="space-y-1 pl-4 border-l border-white/10">
                    <span class="text-[10px] text-slate-500 block">LEVEL 1 • DOMAIN VARIANT (thiserror::Error)</span>
                    <div class="text-slate-300">Caused by: Decryption authentication tag mismatch - memory tampering suspected</div>
                    <div class="text-[11px] text-slate-500">at src/core/crypto.rs:88</div>
                  </div>
                {:else if errorScenario === 'missing_file'}
                  <div class="space-y-1">
                    <span class="text-[10px] text-slate-500 block">LEVEL 0 • APPLICATION CONTEXT (anyhow::Context)</span>
                    <div class="text-white font-medium">Error: Failed to read daemon config file at /etc/ghost/ghost.yaml</div>
                    <div class="text-[11px] text-slate-500">at src/main.rs:188</div>
                  </div>
                  <div class="space-y-1 pl-4 border-l border-white/10">
                    <span class="text-[10px] text-slate-500 block">LEVEL 1 • ROOT KERNEL I/O ERROR (std::io::Error)</span>
                    <div class="text-slate-300">Caused by: No such file or directory (os error 2)</div>
                    <div class="text-[11px] text-slate-500">at std::fs::read()</div>
                  </div>
                {:else}
                  <div class="space-y-1">
                    <span class="text-[10px] text-slate-500 block">LEVEL 0 • APPLICATION CONTEXT (anyhow::Context)</span>
                    <div class="text-white font-medium">Error: Sensor telemetry stream timed out on interface eth0</div>
                    <div class="text-[11px] text-slate-500">at src/perception/xdp_hook.rs:64</div>
                  </div>
                  <div class="space-y-1 pl-4 border-l border-white/10">
                    <span class="text-[10px] text-slate-500 block">LEVEL 1 • RUNTIME ERROR (tokio::time::error::Elapsed)</span>
                    <div class="text-slate-300">Caused by: deadline has elapsed after 5000ms</div>
                    <div class="text-[11px] text-slate-500">at tokio::time::timeout()</div>
                  </div>
                {/if}
              </div>
            </div>
          </div>

          <!-- Code Snippets -->
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div class="p-6 rounded-xl bg-[#0d0e12] border border-white/[0.06] font-mono text-xs space-y-3">
              <div class="text-slate-400 flex justify-between border-b border-white/[0.06] pb-3">
                <span>src/core/crypto.rs</span>
                <span class="text-slate-300 font-medium">thiserror Domain Enum</span>
              </div>
              <pre class="text-slate-300 leading-relaxed overflow-x-auto"><code>use thiserror::Error;

#[derive(Error, Debug)]
pub enum CryptoError &#123;
    #[error("Invalid IV length: expected &#123;0&#125;, got &#123;1&#125;")]
    InvalidIv(usize, usize),
    
    #[error("Auth tag mismatch - RAM tamper suspected")]
    TagMismatch,
&#125;</code></pre>
            </div>

            <div class="p-6 rounded-xl bg-[#0d0e12] border border-white/[0.06] font-mono text-xs space-y-3">
              <div class="text-slate-400 flex justify-between border-b border-white/[0.06] pb-3">
                <span>src/main.rs</span>
                <span class="text-slate-300 font-medium">anyhow Context Bubbling</span>
              </div>
              <pre class="text-slate-300 leading-relaxed overflow-x-auto"><code>use anyhow::&#123;Context, Result&#125;;

pub async fn load_config(path: &str) -&gt; Result&lt;Config&gt; &#123;
    let raw = std::fs::read(path)
        <span class="text-slate-300 font-medium">.with_context(|| format!("Read fail: &#123;path&#125;"))?</span>;
        
    let dec = decrypt(&raw)
        <span class="text-slate-300 font-medium">.context("Crypto integrity check failed")?</span>;
        
    Ok(serde_yaml::from_slice(&dec)?)
&#125;</code></pre>
            </div>
          </div>

        </section>
      {/if}

    </main>

  </div>
</div>

<style>
  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(10px); }
    to { opacity: 1; transform: translateY(0); }
  }
  .animate-fadeIn {
    animation: fadeIn 0.35s cubic-bezier(0.16, 1, 0.3, 1) forwards;
  }
</style>
