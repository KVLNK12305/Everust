<script lang="ts">
  import { onMount } from "svelte";
  import { gsap } from "gsap";
  import { 
    ArrowLeft, Shield, Terminal, Cpu, Zap, Activity, Lock, Unlock, 
    CheckCircle2, AlertTriangle, Layers, Eye, Code2, RefreshCw, 
    FileText, Database, Play, Trash2, Sliders, Check, AlertCircle
  } from "lucide-svelte";

  // ─── Section Navigation & Interactive States ───
  let activeSection = 0;

  const sections = [
    { id: "slice", title: "1. Zero-Copy Slice Pipeline (&[T])", icon: Cpu, badge: "eBPF/XDP" },
    { id: "zeroize", title: "2. Memory Zeroization (zeroize)", icon: Lock, badge: "Crypto" },
    { id: "verdict", title: "3. Exhaustive Verdict Matching", icon: Shield, badge: "ADTs" },
    { id: "serde", title: "4. Resilient Config Fallbacks", icon: Sliders, badge: "Serde" },
    { id: "tokio", title: "5. Async Actor Architecture", icon: Zap, badge: "Tokio" },
    { id: "tracing", title: "6. Structured Telemetry", icon: Terminal, badge: "Tracing" },
    { id: "error", title: "7. Dual-Layer Errors", icon: AlertCircle, badge: "Anyhow" }
  ];

  // ─── 1. Slice vs Vec Interactive Simulator ───
  let simMode: "vec" | "slice" = "slice";
  let allocCount = 0;
  let heapMemoryKb = 0;
  let opsPerSec = 14200;
  let isScanning = false;
  let scanTimer: ReturnType<typeof setInterval> | null = null;

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
          opsPerSec = 3800 + Math.floor(Math.random() * 400);
        } else {
          // Zero-copy slice: no allocations!
          allocCount += 0;
          heapMemoryKb = 0;
          opsPerSec = 24500 + Math.floor(Math.random() * 1200);
        }
      }, 200);
    }
  }

  function resetSim() {
    if (scanTimer) clearInterval(scanTimer);
    isScanning = false;
    allocCount = 0;
    heapMemoryKb = 0;
  }

  // ─── 2. Zeroize Memory Cell Simulator ───
  let memCells = [
    { addr: "0x7F00", val: "4A", label: "IV[0]" },
    { addr: "0x7F01", val: "8F", label: "IV[1]" },
    { addr: "0x7F02", val: "2B", label: "AES_KEY" },
    { addr: "0x7F03", val: "90", label: "AES_KEY" },
    { addr: "0x7F04", val: "E1", label: "AES_KEY" },
    { addr: "0x7F05", val: "CC", label: "AUTH_TAG" },
    { addr: "0x7F06", val: "3D", label: "TOKEN" },
    { addr: "0x7F07", val: "55", label: "TOKEN" }
  ];
  let memState: "active" | "dropped_dead" | "zeroized" = "active";

  function dropWithoutZeroize() {
    memState = "dropped_dead";
  }

  function dropWithZeroize() {
    memState = "zeroized";
    memCells = memCells.map(c => ({ ...c, val: "00" }));
  }

  function resetMemory() {
    memState = "active";
    const orig = ["4A", "8F", "2B", "90", "E1", "CC", "3D", "55"];
    memCells = memCells.map((c, i) => ({ ...c, val: orig[i] }));
  }

  // ─── 3. Exhaustive Verdict Injector ───
  let selectedThreat = { id: "THR-909", confidence: 0.92, severity: "Critical", ip: "198.51.100.24" };
  let currentVerdict = "Action::Neutralize(NeutralizationMethod::Eliminate)";
  let verdictColor = "text-red-400 border-red-500/50 bg-red-500/10";

  function injectThreat(sev: string, conf: number, ip: string) {
    selectedThreat = { id: `THR-${Math.floor(100 + Math.random() * 900)}`, confidence: conf, severity: sev, ip };
    if (conf > 0.8 && sev === "Critical") {
      currentVerdict = "Action::Neutralize(NeutralizationMethod::Eliminate)";
      verdictColor = "text-red-400 border-red-500/50 bg-red-500/10";
    } else if (conf > 0.6 && sev === "High") {
      currentVerdict = "Action::Neutralize(NeutralizationMethod::Isolate)";
      verdictColor = "text-orange-400 border-orange-500/50 bg-orange-500/10";
    } else if (sev === "Medium") {
      currentVerdict = "Action::Probe";
      verdictColor = "text-yellow-400 border-yellow-500/50 bg-yellow-500/10";
    } else {
      currentVerdict = "Action::Ignore";
      verdictColor = "text-slate-400 border-slate-500/50 bg-slate-500/10";
    }
  }

  // ─── 4. Serde Resilient Config Tester ───
  let configScenario: "complete" | "missing_agent" | "malformed" = "missing_agent";

  // ─── 6. Tracing Structured Log Mode ───
  let logMode: "println" | "tracing_json" | "tracing_term" = "tracing_term";

  onMount(() => {
    const tl = gsap.timeline();
    tl.to(".hero-anim", {
      y: 0,
      opacity: 1,
      stagger: 0.1,
      duration: 0.8,
      ease: "power3.out"
    });
  });
</script>

<div class="min-h-screen bg-[#0a0a0a] text-slate-200 px-6 py-16 font-sans selection:bg-rust selection:text-white overflow-x-hidden">
  
  <!-- Ambient Background Glows -->
  <div class="fixed inset-0 pointer-events-none z-0">
    <div class="absolute top-0 left-1/3 w-[600px] h-[600px] bg-rust/10 rounded-full blur-[150px]"></div>
    <div class="absolute bottom-1/4 right-1/4 w-[500px] h-[500px] bg-blue-500/10 rounded-full blur-[150px]"></div>
    <div class="absolute inset-0 bg-[linear-gradient(rgba(255,255,255,0.02)_1px,transparent_1px),linear-gradient(90deg,rgba(255,255,255,0.02)_1px,transparent_1px)] bg-[size:4rem_4rem] [mask-image:radial-gradient(ellipse_60%_50%_at_50%_0%,#000_70%,transparent_100%)]"></div>
  </div>

  <div class="relative z-10 max-w-6xl mx-auto">
    
    <!-- Header / Nav Back -->
    <div class="hero-anim opacity-0 translate-y-4 mb-10 flex items-center justify-between">
      <a href="/experiments" class="inline-flex items-center gap-2 text-slate-400 hover:text-white transition-colors text-sm font-mono no-underline">
        <ArrowLeft size={16} />
        <span>Return to Experiments</span>
      </a>
      <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full border border-rust/40 bg-rust/10 text-rust text-xs font-mono tracking-widest uppercase">
        <span class="w-2 h-2 rounded-full bg-rust animate-ping"></span>
        GhostShell Architecture Ref
      </div>
    </div>

    <!-- Hero Section -->
    <header class="hero-anim opacity-0 translate-y-4 mb-16 border-b border-white/10 pb-12">
      <div class="inline-flex items-center gap-3 text-rust mb-4">
        <Terminal size={22} />
        <span class="font-mono text-xs tracking-[0.2em] uppercase font-semibold">Systems • Security • Daemons</span>
      </div>
      <h1 class="text-5xl md:text-7xl font-bold tracking-tight text-white mb-6">
        Advanced Rust Engineering & <br />
        <span class="text-transparent bg-clip-text bg-gradient-to-r from-rust via-orange-400 to-amber-500">
          eBPF/XDP-Style Patterns
        </span>
      </h1>
      <p class="text-lg md:text-xl text-slate-300 max-w-3xl leading-relaxed font-light mb-8">
        An interactive engineering analysis of the modern Rust design patterns implemented across the <strong class="text-white font-medium">GhostShell</strong> cybersecurity daemon. Bringing kernel-level zero-copy context pipelines, active memory wiping, and exhaustive verdict matching into bare-metal user-space defense.
      </p>

      <div class="flex flex-wrap gap-3">
        {#each sections as sec, idx}
          <button 
            on:click={() => activeSection = idx}
            class="px-4 py-2 rounded-xl border text-sm font-medium flex items-center gap-2.5 transition-all duration-300 {activeSection === idx ? 'bg-rust border-rust text-white shadow-[0_0_20px_rgba(183,65,14,0.4)] scale-105' : 'bg-white/5 border-white/10 text-slate-400 hover:bg-white/10 hover:text-white'}"
          >
            <svelte:component this={sec.icon} size={16} />
            <span>{sec.title.split(" ")[1]} {sec.title.split(" ")[2] || ""}</span>
            <span class="text-[10px] font-mono px-1.5 py-0.5 rounded {activeSection === idx ? 'bg-black/30 text-white' : 'bg-black/40 text-slate-500'}">{sec.badge}</span>
          </button>
        {/each}
      </div>
    </header>

    <!-- Section Content Body -->
    <main class="space-y-16">
      
      <!-- ─── SECTION 1: Zero-Copy Slice Pipeline ─── -->
      {#if activeSection === 0}
        <section class="animate-fadeIn space-y-8">
          <div class="bg-white/[0.03] border border-white/10 rounded-2xl p-8 md:p-10 relative overflow-hidden">
            <div class="absolute top-0 right-0 w-96 h-96 bg-blue-500/5 rounded-full blur-[100px] pointer-events-none"></div>
            
            <div class="flex items-center gap-3 mb-4">
              <div class="p-2.5 bg-blue-500/10 text-blue-400 rounded-xl">
                <Cpu size={24} />
              </div>
              <h2 class="text-3xl font-bold text-white">1. The eBPF/XDP-Style Zero-Copy Slice Pipeline (<code class="text-blue-400 font-mono">&[T]</code>)</h2>
            </div>

            <p class="text-slate-300 leading-relaxed mb-8 max-w-4xl">
              Instead of allocating vectors (<code class="text-rust font-mono">Vec&lt;T&gt;</code>) or passing owned objects around, evaluation and anomaly detection loops pass <strong class="text-white">immutable borrowed slices (<code class="text-blue-400 font-mono">&[T]</code>)</strong>. In Linux kernel networking, eXpress Data Path (XDP) hooks receive a lightweight pointer to packet metadata (<code class="text-slate-400 font-mono">struct xdp_md *ctx</code>). The packet is never copied; verification filters inspect memory directly via pointer bounds.
            </p>

            <!-- Interactive Simulation Lab -->
            <div class="bg-black/60 border border-white/10 rounded-xl p-6 mb-8">
              <div class="flex flex-col md:flex-row items-start md:items-center justify-between gap-4 mb-6 pb-6 border-b border-white/10">
                <div>
                  <h3 class="text-lg font-bold text-white flex items-center gap-2">
                    <Activity class="text-rust" size={20} />
                    Live Allocator & Throughput Laboratory
                  </h3>
                  <p class="text-xs text-slate-400">Compare heap allocation pressure during high-frequency threat scanning loops.</p>
                </div>
                <div class="flex items-center gap-3">
                  <div class="bg-white/5 p-1 rounded-lg border border-white/10 flex">
                    <button 
                      on:click={() => { simMode = "vec"; resetSim(); }}
                      class="px-3 py-1.5 rounded text-xs font-mono font-medium transition-all {simMode === 'vec' ? 'bg-red-500/20 text-red-400 border border-red-500/30' : 'text-slate-400 hover:text-white'}"
                    >
                      Vec&lt;Threat&gt; (Heap Clone)
                    </button>
                    <button 
                      on:click={() => { simMode = "slice"; resetSim(); }}
                      class="px-3 py-1.5 rounded text-xs font-mono font-medium transition-all {simMode === 'slice' ? 'bg-blue-500/20 text-blue-400 border border-blue-500/30' : 'text-slate-400 hover:text-white'}"
                    >
                      &[Threat] (Zero-Copy)
                    </button>
                  </div>
                  <button 
                    on:click={toggleScan}
                    class="px-4 py-1.5 rounded-lg font-mono text-xs font-bold flex items-center gap-2 transition-all {isScanning ? 'bg-amber-500/20 text-amber-400 border border-amber-500/40' : 'bg-emerald-500/20 text-emerald-400 border border-emerald-500/40'}"
                  >
                    <Play size={14} class={isScanning ? "animate-spin" : ""} />
                    <span>{isScanning ? "PAUSE SCAN" : "START SCAN"}</span>
                  </button>
                </div>
              </div>

              <!-- Metrics Display -->
              <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                <div class="bg-white/5 border border-white/5 rounded-xl p-4">
                  <span class="text-xs font-mono text-slate-500 block mb-1">HEAP ALLOCATIONS / SEC</span>
                  <div class="text-3xl font-mono font-bold {simMode === 'vec' ? 'text-red-400' : 'text-blue-400'}">
                    {allocCount} <span class="text-xs font-normal text-slate-500">allocs</span>
                  </div>
                  <div class="mt-2 text-[11px] text-slate-400">
                    {simMode === 'vec' ? '⚠️ High malloc/free pressure' : '⚡ 0 heap allocations (Stack pointer)'}
                  </div>
                </div>

                <div class="bg-white/5 border border-white/5 rounded-xl p-4">
                  <span class="text-xs font-mono text-slate-500 block mb-1">MEMORY FRAGMENTATION</span>
                  <div class="text-3xl font-mono font-bold {simMode === 'vec' ? 'text-amber-400' : 'text-emerald-400'}">
                    {heapMemoryKb} <span class="text-xs font-normal text-slate-500">KB churn</span>
                  </div>
                  <div class="mt-2 text-[11px] text-slate-400">
                    {simMode === 'vec' ? '⚠️ GC / Drop overhead active' : '⚡ Zero memory footprint growth'}
                  </div>
                </div>

                <div class="bg-white/5 border border-white/5 rounded-xl p-4">
                  <span class="text-xs font-mono text-slate-500 block mb-1">EVALUATION THROUGHPUT</span>
                  <div class="text-3xl font-mono font-bold text-white">
                    {opsPerSec.toLocaleString()} <span class="text-xs font-normal text-slate-500">ops/sec</span>
                  </div>
                  <div class="mt-2 text-[11px] text-slate-400">
                    {simMode === 'vec' ? '🐢 Constrained by memory bus' : '🚀 Bare-metal L1 cache velocity'}
                  </div>
                </div>
              </div>
            </div>

            <!-- Code Comparison -->
            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
              <div class="bg-black/80 border border-white/10 rounded-xl p-5 font-mono text-xs">
                <div class="text-slate-500 mb-3 pb-2 border-b border-white/10 flex justify-between">
                  <span>❌ Traditional Heap Allocation</span>
                  <span class="text-red-400">Vec&lt;T&gt;</span>
                </div>
                <pre class="text-slate-300 overflow-x-auto"><code><span class="text-slate-500">// Requires cloning or ownership transfer</span>
pub fn evaluate(&mut self, threats: <span class="text-red-400">Vec&lt;Threat&gt;</span>) -> Vec&lt;Action&gt; &#123;
    let mut actions = Vec::new();
    for threat in threats &#123; <span class="text-slate-500">// Moves ownership</span>
        if threat.confidence > 0.8 &#123;
            actions.push(Action::Neutralize);
        &#125;
    &#125;
    actions <span class="text-slate-500">// Heap deallocation delay</span>
&#125;</code></pre>
              </div>

              <div class="bg-black/80 border border-blue-500/30 rounded-xl p-5 font-mono text-xs">
                <div class="text-blue-400 mb-3 pb-2 border-b border-white/10 flex justify-between font-bold">
                  <span>⚡ XDP Zero-Copy Slice Pipeline</span>
                  <span>&[T]</span>
                </div>
                <pre class="text-slate-200 overflow-x-auto"><code><span class="text-slate-500">// Borrowed contiguous slice (xdp_md *ctx equivalent)</span>
pub fn evaluate(&mut self, threats: <span class="text-blue-400">&[Threat]</span>) -> Vec&lt;Action&gt; &#123;
    let mut actions = Vec::with_capacity(threats.len());
    for threat in threats &#123; <span class="text-slate-500">// Direct pointer inspection</span>
        if threat.confidence > 0.8 &#123;
            actions.push(Action::Neutralize);
        &#125;
    &#125;
    actions
&#125;</code></pre>
              </div>
            </div>

          </div>
        </section>
      {/if}

      <!-- ─── SECTION 2: Active Memory Zeroization ─── -->
      {#if activeSection === 1}
        <section class="animate-fadeIn space-y-8">
          <div class="bg-white/[0.03] border border-white/10 rounded-2xl p-8 md:p-10 relative overflow-hidden">
            <div class="flex items-center gap-3 mb-4">
              <div class="p-2.5 bg-rust/10 text-rust rounded-xl">
                <Lock size={24} />
              </div>
              <h2 class="text-3xl font-bold text-white">2. Active Memory Zeroization & Cryptographic Hygiene (<code class="text-rust font-mono">zeroize</code>)</h2>
            </div>

            <p class="text-slate-300 leading-relaxed mb-8 max-w-4xl">
              In standard languages, sensitive strings (encryption keys, tokens) remain in RAM even after going out of scope until the OS reclaims or overwrites the page. In Rust, if you write <code class="text-slate-400 font-mono">buffer.fill(0)</code> right before drop, LLVM optimizing compilers notice the variable is never read again and <strong class="text-red-400">delete your zeroing loop as a "dead store optimization."</strong>
            </p>

            <!-- Memory Cell Lab -->
            <div class="bg-black/70 border border-white/10 rounded-xl p-6 mb-8">
              <div class="flex flex-col md:flex-row items-start md:items-center justify-between gap-4 mb-6 pb-4 border-b border-white/10">
                <div>
                  <h3 class="text-lg font-bold text-white flex items-center gap-2">
                    <Database class="text-rust" size={18} />
                    RAM Page Inspection Bench (Address Space 0x7F00 - 0x7F07)
                  </h3>
                  <p class="text-xs text-slate-400">Watch physical RAM behavior when sensitive key variables leave scope.</p>
                </div>
                <div class="flex gap-2">
                  <button 
                    on:click={resetMemory}
                    class="px-3 py-1.5 bg-white/5 hover:bg-white/10 border border-white/10 text-slate-300 rounded text-xs font-mono flex items-center gap-1.5 transition-colors"
                  >
                    <RefreshCw size={12} /> Reset Buffer
                  </button>
                  <button 
                    on:click={dropWithoutZeroize}
                    class="px-3 py-1.5 bg-red-500/10 hover:bg-red-500/20 border border-red-500/30 text-red-400 rounded text-xs font-mono transition-colors"
                  >
                    Drop Without Zeroize
                  </button>
                  <button 
                    on:click={dropWithZeroize}
                    class="px-3 py-1.5 bg-emerald-500/20 hover:bg-emerald-500/30 border border-emerald-500/40 text-emerald-300 rounded text-xs font-mono font-bold flex items-center gap-1.5 transition-colors"
                  >
                    <CheckCircle2 size={14} /> Drop with zeroize::Zeroize
                  </button>
                </div>
              </div>

              <!-- Hex Cells Grid -->
              <div class="grid grid-cols-2 sm:grid-cols-4 md:grid-cols-8 gap-3 mb-6">
                {#each memCells as cell}
                  <div class="bg-black/90 border {memState === 'zeroized' ? 'border-emerald-500/40 bg-emerald-950/10' : (memState === 'dropped_dead' ? 'border-red-500/50 bg-red-950/20 animate-pulse' : 'border-white/10')} rounded-lg p-3 text-center transition-all duration-300">
                    <div class="text-[10px] font-mono text-slate-500 mb-1">{cell.addr}</div>
                    <div class="text-2xl font-mono font-bold {memState === 'zeroized' ? 'text-emerald-400' : (memState === 'dropped_dead' ? 'text-red-400' : 'text-white')}">{cell.val}</div>
                    <div class="text-[10px] font-mono mt-1 text-slate-400 truncate">{cell.label}</div>
                  </div>
                {/each}
              </div>

              <!-- Status Feedback -->
              <div class="p-4 rounded-lg border text-sm font-mono {memState === 'zeroized' ? 'bg-emerald-500/10 border-emerald-500/30 text-emerald-300' : (memState === 'dropped_dead' ? 'bg-red-500/10 border-red-500/30 text-red-300' : 'bg-white/5 border-white/10 text-slate-300')}">
                {#if memState === 'active'}
                  ℹ️ Status: Sensitive cryptographic AES-256-GCM keys currently allocated in physical RAM.
                {:else if memState === 'dropped_dead'}
                  ⚠️ CRITICAL VULNERABILITY: Variable dropped from scope, but LLVM Dead-Store Optimization deleted manual memory cleanup. Secret keys remain readable in RAM dumps!
                {:else}
                  🛡️ CRYPTOGRAPHIC HYGIENE VERIFIED: volatile memory writes & compiler fences forced physical CPU overwrites to 0x00 before deallocation.
                {/if}
              </div>
            </div>

            <div class="bg-black/80 border border-white/10 rounded-xl p-5 font-mono text-xs">
              <div class="text-slate-400 mb-3 pb-2 border-b border-white/10 flex justify-between">
                <span>src/core/crypto.rs</span>
                <span class="text-rust font-bold">RAII Drop Trait Binding</span>
              </div>
              <pre class="text-slate-200 overflow-x-auto"><code>use zeroize::Zeroize;

#[derive(Debug, Clone)]
pub struct SecureMemory &#123;
    buffer: Vec&lt;u8&gt;,
&#125;

impl Drop for SecureMemory &#123;
    fn drop(&mut self) &#123;
        <span class="text-emerald-400">// Volatile write barriers defeat LLVM dead-store elimination</span>
        self.buffer.zeroize();
    &#125;
&#125;</code></pre>
            </div>

          </div>
        </section>
      {/if}

      <!-- ─── SECTION 3: Exhaustive Verdict Matching ─── -->
      {#if activeSection === 2}
        <section class="animate-fadeIn space-y-8">
          <div class="bg-white/[0.03] border border-white/10 rounded-2xl p-8 md:p-10 relative overflow-hidden">
            <div class="flex items-center gap-3 mb-4">
              <div class="p-2.5 bg-emerald-500/10 text-emerald-400 rounded-xl">
                <Shield size={24} />
              </div>
              <h2 class="text-3xl font-bold text-white">3. Verdict-Driven Countermeasures via Exhaustive Enum Matching</h2>
            </div>

            <p class="text-slate-300 leading-relaxed mb-8 max-w-4xl">
              In XDP, every packet evaluation must terminate in an explicit kernel verdict (<code class="text-slate-400 font-mono">XDP_PASS</code>, <code class="text-slate-400 font-mono">XDP_DROP</code>, <code class="text-slate-400 font-mono">XDP_TX</code>). In GhostShell, every threat evaluated by the <code class="text-emerald-400 font-mono">ActionEngine</code> must resolve to an <code class="text-white font-mono">Action</code> Algebraic Data Type (ADT) enum variant. Rust's <code class="text-rust font-mono">match</code> operator enforces exhaustive compile-time coverage.
            </p>

            <!-- Verdict Laboratory -->
            <div class="bg-black/60 border border-white/10 rounded-xl p-6 mb-8">
              <h3 class="text-lg font-bold text-white mb-4 flex items-center gap-2">
                <Layers class="text-emerald-400" size={18} />
                Interactive Threat Verdict Dispatcher
              </h3>

              <div class="flex flex-wrap gap-2 mb-6">
                <button 
                  on:click={() => injectThreat("Critical", 0.95, "198.51.100.24")}
                  class="px-3 py-1.5 rounded text-xs font-mono font-bold bg-red-500/20 border border-red-500/40 text-red-300 hover:bg-red-500/30 transition-colors"
                >
                  Inject Critical (0.95 conf)
                </button>
                <button 
                  on:click={() => injectThreat("High", 0.75, "203.0.113.88")}
                  class="px-3 py-1.5 rounded text-xs font-mono font-bold bg-orange-500/20 border border-orange-500/40 text-orange-300 hover:bg-orange-500/30 transition-colors"
                >
                  Inject High (0.75 conf)
                </button>
                <button 
                  on:click={() => injectThreat("Medium", 0.50, "192.0.2.10")}
                  class="px-3 py-1.5 rounded text-xs font-mono font-bold bg-yellow-500/20 border border-yellow-500/40 text-yellow-300 hover:bg-yellow-500/30 transition-colors"
                >
                  Inject Medium (0.50 conf)
                </button>
                <button 
                  on:click={() => injectThreat("Low", 0.15, "10.0.0.4")}
                  class="px-3 py-1.5 rounded text-xs font-mono font-bold bg-slate-500/20 border border-slate-500/40 text-slate-300 hover:bg-slate-500/30 transition-colors"
                >
                  Inject Low / Benign
                </button>
              </div>

              <!-- Dispatch Visualization -->
              <div class="grid grid-cols-1 md:grid-cols-2 gap-4 items-center bg-white/5 p-5 rounded-xl border border-white/5 font-mono text-xs">
                <div class="space-y-2 border-r border-white/10 pr-4">
                  <span class="text-slate-500 block">INCOMING THREAT METADATA:</span>
                  <div class="text-white font-bold text-sm">ID: {selectedThreat.id}</div>
                  <div>Severity: <span class="font-bold text-white">{selectedThreat.severity}</span></div>
                  <div>Confidence: <span class="text-emerald-400">{(selectedThreat.confidence * 100).toFixed(0)}%</span></div>
                  <div>Source IP: <span class="text-slate-300">{selectedThreat.ip}</span></div>
                </div>

                <div class="pl-2 space-y-2">
                  <span class="text-slate-500 block">RESOLVED ALGEBRAIC VERDICT:</span>
                  <div class="p-3 rounded-lg border font-bold text-sm {verdictColor} transition-all">
                    {currentVerdict}
                  </div>
                  <div class="text-[11px] text-slate-400 mt-2">
                    ⚡ Compile-time guarantee: no unhandled cases or null-pointer fallthroughs allowed.
                  </div>
                </div>
              </div>
            </div>

          </div>
        </section>
      {/if}

      <!-- ─── SECTION 4: Resilient Configuration ─── -->
      {#if activeSection === 3}
        <section class="animate-fadeIn space-y-8">
          <div class="bg-white/[0.03] border border-white/10 rounded-2xl p-8 md:p-10 relative overflow-hidden">
            <div class="flex items-center gap-3 mb-4">
              <div class="p-2.5 bg-amber-500/10 text-amber-400 rounded-xl">
                <Sliders size={24} />
              </div>
              <h2 class="text-3xl font-bold text-white">4. Resilient Configuration with Default Fallbacks (<code class="text-amber-400 font-mono">#[serde(default)]</code>)</h2>
            </div>

            <p class="text-slate-300 leading-relaxed mb-8 max-w-4xl">
              When software evolves, new configuration fields are added (e.g., <code class="text-slate-400 font-mono">max_concurrent_ops: 10</code>). If an existing server runs an older <code class="text-slate-400 font-mono">/etc/ghost/ghost.yaml</code> lacking this key, standard parsers crash on startup. By annotating structs with <code class="text-amber-400 font-mono">#[serde(default)]</code>, Serde calls <code class="text-white font-mono">Self::default()</code> for missing fields, enabling zero-downtime upgrades.
            </p>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
              <div class="bg-black/80 border border-white/10 rounded-xl p-5 font-mono text-xs">
                <div class="text-slate-400 mb-3 pb-2 border-b border-white/10 flex justify-between">
                  <span>Legacy YAML File (v0.1.0 schema)</span>
                  <span class="text-amber-400">Missing Fields</span>
                </div>
                <pre class="text-slate-300"><code>daemon_name: "ghostshell-node-01"
# Note: agent & stealth config blocks
# are completely missing from this older file!
</code></pre>
              </div>

              <div class="bg-black/80 border border-emerald-500/30 rounded-xl p-5 font-mono text-xs">
                <div class="text-emerald-400 mb-3 pb-2 border-b border-white/10 flex justify-between font-bold">
                  <span>Deser Result in RAM (v0.2.0 Daemon)</span>
                  <span>100% Valid</span>
                </div>
                <pre class="text-emerald-300"><code>Config &#123;
    daemon_name: "ghostshell-node-01",
    agent: AgentConfig &#123;
        max_concurrent_ops: 10, // Injected via Default!
        scan_interval_ms: 1000, // Injected via Default!
        strict_mode: true
    &#125;
&#125;</code></pre>
              </div>
            </div>
          </div>
        </section>
      {/if}

      <!-- ─── SECTION 5: Async Actor Architecture ─── -->
      {#if activeSection === 4}
        <section class="animate-fadeIn space-y-8">
          <div class="bg-white/[0.03] border border-white/10 rounded-2xl p-8 md:p-10 relative overflow-hidden">
            <div class="flex items-center gap-3 mb-4">
              <div class="p-2.5 bg-yellow-500/10 text-yellow-400 rounded-xl">
                <Zap size={24} />
              </div>
              <h2 class="text-3xl font-bold text-white">5. Asynchronous Multi-Engine Actor Architecture (<code class="text-yellow-400 font-mono">tokio</code> & Futures)</h2>
            </div>

            <p class="text-slate-300 leading-relaxed mb-8 max-w-4xl">
              GhostShell operates as an asynchronous daemon where each core engine acts like a decoupled concurrent service. Because <code class="text-yellow-400 font-mono">tokio</code> manages a work-stealing thread pool, the daemon can concurrently monitor 10,000 network connections, read Linux <code class="text-slate-400 font-mono">/proc</code> filesystem trees, and encrypt telemetry reports without consuming excessive OS threads or blocking kernel I/O.
            </p>

            <div class="bg-black/80 border border-white/10 rounded-xl p-5 font-mono text-xs">
              <div class="text-slate-400 mb-3 pb-2 border-b border-white/10 flex justify-between">
                <span>src/agent/mod.rs</span>
                <span class="text-yellow-400">Cooperative Futures Loop</span>
              </div>
              <pre class="text-slate-200 overflow-x-auto"><code>impl GhostAgent &#123;
    pub async fn run(&mut self) -&gt; Result&lt;()&gt; &#123;
        <span class="text-slate-500">// Non-blocking monitoring loop yielding to Tokio runtime</span>
        loop &#123;
            let threats = self.perception.scan().await?;
            if !threats.is_empty() &#123;
                self.actions.execute_all(&threats).await?;
            &#125;
            tokio::time::sleep(Duration::from_secs(5)).await;
        &#125;
    &#125;
&#125;</code></pre>
            </div>
          </div>
        </section>
      {/if}

      <!-- ─── SECTION 6: Structured Telemetry ─── -->
      {#if activeSection === 5}
        <section class="animate-fadeIn space-y-8">
          <div class="bg-white/[0.03] border border-white/10 rounded-2xl p-8 md:p-10 relative overflow-hidden">
            <div class="flex items-center gap-3 mb-4">
              <div class="p-2.5 bg-cyan-500/10 text-cyan-400 rounded-xl">
                <Terminal size={24} />
              </div>
              <h2 class="text-3xl font-bold text-white">6. Enterprise Structured Telemetry (<code class="text-cyan-400 font-mono">tracing</code> vs <code class="text-red-400 font-mono">println!</code>)</h2>
            </div>

            <p class="text-slate-300 leading-relaxed mb-8 max-w-4xl">
              <code class="text-red-400 font-mono">println!</code> is synchronous and locks standard output on every invocation, creating severe bottlenecks in high-throughput monitoring tools. Rust's <code class="text-cyan-400 font-mono">tracing</code> framework records events as structured key-value pairs (<code class="text-slate-400 font-mono">threat_id = %threat.id</code>), allowing formatters to emit clean JSON blobs for SIEM pipelines (Elasticsearch, Splunk) with zero-cost runtime filtering.
            </p>

            <!-- Log Mode Switcher -->
            <div class="bg-black/70 border border-white/10 rounded-xl p-6 mb-6">
              <div class="flex justify-between items-center mb-4 pb-3 border-b border-white/10">
                <span class="text-sm font-bold text-white font-mono">TELEMETRY INGESTION FORMATTER BENCH</span>
                <div class="flex gap-2">
                  <button 
                    on:click={() => logMode = "println"}
                    class="px-3 py-1 rounded text-xs font-mono {logMode === 'println' ? 'bg-red-500/20 text-red-400 border border-red-500/40' : 'text-slate-400 hover:text-white'}"
                  >
                    println! (Legacy Text)
                  </button>
                  <button 
                    on:click={() => logMode = "tracing_term"}
                    class="px-3 py-1 rounded text-xs font-mono {logMode === 'tracing_term' ? 'bg-cyan-500/20 text-cyan-400 border border-cyan-500/40' : 'text-slate-400 hover:text-white'}"
                  >
                    tracing (Terminal)
                  </button>
                  <button 
                    on:click={() => logMode = "tracing_json"}
                    class="px-3 py-1 rounded text-xs font-mono {logMode === 'tracing_json' ? 'bg-emerald-500/20 text-emerald-400 border border-emerald-500/40' : 'text-slate-400 hover:text-white'}"
                  >
                    tracing (SIEM JSON Blob)
                  </button>
                </div>
              </div>

              <div class="bg-black/90 p-4 rounded-lg font-mono text-xs overflow-x-auto border border-white/5">
                {#if logMode === 'println'}
                  <div class="text-red-300">Neutralizing active threat THR-001 with Eliminate</div>
                  <div class="text-slate-500 mt-2 text-[11px]">// ⚠️ Unparseable raw string, locks stdout mutex synchronously</div>
                {:else if logMode === 'tracing_term'}
                  <div class="text-cyan-300">2026-07-01T17:45:53.860863Z  INFO Executing XDP-style neutralization threat_id=THR-001 method=Eliminate</div>
                  <div class="text-slate-500 mt-2 text-[11px]">// ⚡ Async structured fields, dynamically filtered via RUST_LOG env</div>
                {:else}
                  <div class="text-emerald-300">
                    &#123;"timestamp":"2026-07-01T17:45:53.860863Z","level":"INFO","fields":&#123;"threat_id":"THR-001","method":"Eliminate","message":"Executing XDP-style neutralization"&#125;&#125;
                  </div>
                  <div class="text-slate-500 mt-2 text-[11px]">// 🚀 Machine-readable SIEM JSON blob ready for Elasticsearch / Splunk ingestion</div>
                {/if}
              </div>
            </div>

          </div>
        </section>
      {/if}

      <!-- ─── SECTION 7: Dual-Layer Errors ─── -->
      {#if activeSection === 6}
        <section class="animate-fadeIn space-y-8">
          <div class="bg-white/[0.03] border border-white/10 rounded-2xl p-8 md:p-10 relative overflow-hidden">
            <div class="flex items-center gap-3 mb-4">
              <div class="p-2.5 bg-red-500/10 text-red-400 rounded-xl">
                <AlertCircle size={24} />
              </div>
              <h2 class="text-3xl font-bold text-white">7. Dual-Layer Error Propagation (<code class="text-red-400 font-mono">thiserror</code> + <code class="text-orange-400 font-mono">anyhow</code>)</h2>
            </div>

            <p class="text-slate-300 leading-relaxed mb-8 max-w-4xl">
              GhostShell combines two paradigms: <strong class="text-white font-mono">thiserror</strong> inside library modules generates exact domain errors without runtime overhead for programmatic matching. <strong class="text-white font-mono">anyhow</strong> is used in application entry points and orchestration loops to capture backtraces and attach rich human-readable context via <code class="text-slate-400 font-mono">.context("...")</code> using the ergonomic <code class="text-rust font-mono">?</code> operator.
            </p>

            <div class="bg-black/80 border border-white/10 rounded-xl p-5 font-mono text-xs">
              <div class="text-slate-400 mb-3 pb-2 border-b border-white/10 flex justify-between">
                <span>Unified Error Trace Causality Chain</span>
                <span class="text-red-400">? Operator Magic</span>
              </div>
              <pre class="text-slate-200 overflow-x-auto"><code>Error: Cryptographic integrity check failed while loading configuration

Caused by:
    0: Decryption authentication tag mismatch - memory tampering suspected
    1: OS I/O error: No such file or directory (os error 2)</code></pre>
            </div>
          </div>
        </section>
      {/if}

    </main>

    <!-- Footer Quote -->
    <footer class="mt-20 pt-10 border-t border-white/10 text-center">
      <div class="inline-block p-6 rounded-2xl bg-white/[0.02] border border-white/5 max-w-4xl">
        <p class="text-sm md:text-base text-slate-300 italic font-light mb-4">
          "GhostShell demonstrates how modern Rust allows us to bring kernel-level design philosophies—like XDP/eBPF zero-copy context pipelines and exhaustive verdict matching—into user-space cybersecurity daemons. By pairing Rust's ownership model with active memory zeroization (<code class="text-rust not-italic font-mono">zeroize</code>), resilient serialization (<code class="text-amber-400 not-italic font-mono">serde</code>), and asynchronous actor loops (<code class="text-yellow-400 not-italic font-mono">tokio</code>), we achieve memory safety, zero-downtime reliability, and bare-metal execution speed."
        </p>
        <span class="text-xs font-mono text-rust tracking-widest uppercase">— GhostShell Engineering Manifesto</span>
      </div>
    </footer>

  </div>
</div>

<style>
  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(10px); }
    to { opacity: 1; transform: translateY(0); }
  }
  .animate-fadeIn {
    animation: fadeIn 0.4s cubic-bezier(0.16, 1, 0.3, 1) forwards;
  }
</style>
