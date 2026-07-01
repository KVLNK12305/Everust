<script lang="ts">
  import { onMount } from "svelte";
  import { gsap } from "gsap";
  import { 
    ArrowLeft, Shield, Layers, Code, Cpu, Check, AlertCircle, 
    Play, RefreshCw, ChevronRight
  } from "lucide-svelte";

  // ─── Navigation State ───
  let activeTab = 0;
  const tabs = [
    { id: "move", num: "01", title: "Move Semantics", sub: "Stack vs Heap Transfer", icon: Cpu },
    { id: "borrow", num: "02", title: "Borrow Checker", sub: "Reference Aliasing Matrix", icon: Shield },
    { id: "slice", num: "03", title: "Slice Views", sub: "Zero-Allocation Pointers", icon: Layers },
    { id: "raii", num: "04", title: "RAII & Scopes", sub: "Deterministic Unwinding", icon: Code }
  ];

  // ─── 1. Move Semantics Lab State ───
  let memoryScenario: "init" | "moved" | "cloned" | "copy" = "init";
  
  // ─── 2. Borrow Checker Lab State ───
  let immutCount = 2;
  let mutActive = false;
  let borrowError: string | null = null;

  function addImmutBorrow() {
    if (mutActive) {
      borrowError = "error[E0502]: cannot borrow `buffer` as immutable because it is also borrowed as mutable";
    } else {
      immutCount++;
      borrowError = null;
    }
  }

  function toggleMutBorrow() {
    if (mutActive) {
      mutActive = false;
      borrowError = null;
    } else if (immutCount > 0) {
      borrowError = "error[E0502]: cannot borrow `buffer` as mutable because it is also borrowed as immutable";
    } else {
      mutActive = true;
      borrowError = null;
    }
  }

  function resetBorrows() {
    immutCount = 0;
    mutActive = false;
    borrowError = null;
  }

  // ─── 3. Slice Offset Lab State ───
  let sliceStart = 0;
  let sliceEnd = 6;
  const rawString = "SYS_OK:CPU_LOAD_12%:MEM_OK";

  // ─── 4. RAII Scoping State ───
  let scopeStep: 0 | 1 | 2 | 3 = 0;
  const raiiStack = [
    { name: "res1", type: "NetworkSocket", addr: "0x7F00", status: "Allocated" },
    { name: "res2", type: "FileDescriptor", addr: "0x7F08", status: "Allocated" },
    { name: "res3", type: "MemoryBuffer", addr: "0x7F10", status: "Allocated" }
  ];

  onMount(() => {
    gsap.to(".fade-in-up", {
      y: 0,
      opacity: 1,
      stagger: 0.08,
      duration: 0.5,
      ease: "power2.out"
    });
  });
</script>

<div class="min-h-screen bg-[#090a0d] text-slate-300 font-sans selection:bg-rust/80 selection:text-white pb-32">
  
  <!-- Subtle Sticky Header -->
  <header class="border-b border-white/[0.06] bg-[#090a0d]/90 backdrop-blur-md sticky top-0 z-40">
    <div class="max-w-5xl mx-auto px-6 h-16 flex items-center justify-between">
      <div class="flex items-center gap-6">
        <a href="/foundations" class="inline-flex items-center gap-2 text-xs font-mono text-slate-400 hover:text-white transition-colors no-underline">
          <ArrowLeft size={14} />
          <span>Foundations</span>
        </a>
        <div class="h-4 w-px bg-white/10"></div>
        <span class="text-xs font-mono text-slate-400">Module 01 • Ownership & Borrowing</span>
      </div>
      <div class="flex items-center gap-3 text-xs font-mono text-slate-400">
        <span class="px-2 py-0.5 rounded bg-white/5 border border-white/10 text-slate-300">Rust 2024</span>
      </div>
    </div>
  </header>

  <div class="max-w-5xl mx-auto px-6 pt-16">
    
    <!-- Hero Title Section -->
    <div class="fade-in-up opacity-0 translate-y-3 mb-16 space-y-4">
      <div class="inline-block px-3 py-1 rounded-md bg-white/[0.04] border border-white/10 text-xs font-mono text-rust">
        MODULE 01 • MEMORY MODEL
      </div>
      <h1 class="text-4xl md:text-5xl font-bold text-white tracking-tight leading-tight">
        Ownership & Move Semantics
      </h1>
      <p class="text-base md:text-lg text-slate-400 max-w-2xl font-light leading-relaxed">
        Rust achieves memory safety and concurrency guarantees without a garbage collector through its single-owner memory model, compile-time borrow checking, and zero-allocation slice fat pointers.
      </p>
    </div>

    <!-- Section Tabs -->
    <div class="fade-in-up opacity-0 translate-y-3 grid grid-cols-2 sm:grid-cols-4 gap-3 mb-16">
      {#each tabs as tab, idx}
        <button
          on:click={() => activeTab = idx}
          class="text-left p-4 rounded-xl border transition-all duration-200 {activeTab === idx ? 'bg-[#13161c] border-white/20 text-white shadow-lg' : 'bg-[#0d0e12]/60 border-white/[0.06] text-slate-400 hover:border-white/10 hover:text-slate-200 hover:bg-[#0f1116]'}"
        >
          <div class="flex items-center justify-between mb-3 text-xs font-mono">
            <span class="{activeTab === idx ? 'text-rust' : 'text-slate-500'}">{tab.num}</span>
            <svelte:component this={tab.icon} size={16} class={activeTab === idx ? 'text-slate-200' : 'text-slate-500'} />
          </div>
          <div class="font-medium text-xs md:text-sm truncate mb-0.5">
            {tab.title}
          </div>
          <div class="text-[11px] text-slate-500 truncate">
            {tab.sub}
          </div>
        </button>
      {/each}
    </div>

    <main class="min-h-[500px]">
      
      <!-- ════════════════════════════════════════════════════════════════ -->
      <!-- TAB 01: Move Semantics                                           -->
      <!-- ════════════════════════════════════════════════════════════════ -->
      {#if activeTab === 0}
        <section class="space-y-12 animate-fadeIn">
          
          <div class="space-y-4 max-w-3xl">
            <h2 class="text-2xl md:text-3xl font-bold text-white tracking-tight">
              Move Semantics vs Stack Copying
            </h2>
            <p class="text-slate-400 leading-relaxed text-base">
              In Rust, every value has a single owner. When assigning a heap-allocated variable (such as <code class="text-slate-300 font-mono">String</code> or <code class="text-slate-300 font-mono">Vec</code>) to another variable, ownership is transferred (a <em>move</em>). The compiler invalidates the source variable's stack metadata to prevent double-free bugs during deallocation. Primitive types implementing the <code class="text-slate-300 font-mono">Copy</code> trait (like <code class="text-slate-300 font-mono">i32</code> or <code class="text-slate-300 font-mono">bool</code>) duplicate their stack bits without transferring ownership.
            </p>
          </div>

          <!-- Interactive Memory Inspector -->
          <div class="p-8 rounded-2xl bg-[#0d0e12] border border-white/[0.08] space-y-8">
            <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 pb-6 border-b border-white/[0.06]">
              <div>
                <h3 class="font-medium text-white text-base">Stack & Heap Memory Map</h3>
                <p class="text-xs text-slate-400 mt-1">Select an assignment operation to inspect memory pointer ownership.</p>
              </div>
              <div class="flex flex-wrap gap-2 text-xs font-mono">
                <button
                  on:click={() => memoryScenario = "init"}
                  class="px-3 py-1.5 rounded transition-all {memoryScenario === 'init' ? 'bg-white/10 text-white' : 'bg-white/5 text-slate-400 hover:text-slate-200'}"
                >
                  let s1 = String::from("Rust");
                </button>
                <button
                  on:click={() => memoryScenario = "moved"}
                  class="px-3 py-1.5 rounded transition-all {memoryScenario === 'moved' ? 'bg-white/10 text-white border border-rust/40 text-rust' : 'bg-white/5 text-slate-400 hover:text-slate-200'}"
                >
                  let s2 = s1; (Move)
                </button>
                <button
                  on:click={() => memoryScenario = "cloned"}
                  class="px-3 py-1.5 rounded transition-all {memoryScenario === 'cloned' ? 'bg-white/10 text-white' : 'bg-white/5 text-slate-400 hover:text-slate-200'}"
                >
                  let s2 = s1.clone(); (Deep Copy)
                </button>
                <button
                  on:click={() => memoryScenario = "copy"}
                  class="px-3 py-1.5 rounded transition-all {memoryScenario === 'copy' ? 'bg-white/10 text-white' : 'bg-white/5 text-slate-400 hover:text-slate-200'}"
                >
                  let y = x; (Primitive Copy)
                </button>
              </div>
            </div>

            <!-- Memory Visualization Grid -->
            <div class="grid grid-cols-1 md:grid-cols-2 gap-8 font-mono text-xs">
              
              <!-- Stack Frame Box -->
              <div class="p-5 rounded-xl bg-black/40 border border-white/[0.05] space-y-4">
                <span class="text-slate-500 block text-[11px]">STACK FRAME METADATA (LIFO)</span>
                
                {#if memoryScenario === "copy"}
                  <div class="p-4 rounded-lg bg-white/[0.02] border border-white/10 space-y-2">
                    <div class="flex justify-between text-slate-300"><span>Variable: <strong>x</strong></span><span class="text-slate-500">i32</span></div>
                    <div class="text-emerald-400 text-sm">Value: 42</div>
                  </div>
                  <div class="p-4 rounded-lg bg-white/[0.02] border border-emerald-500/30 space-y-2">
                    <div class="flex justify-between text-slate-300"><span>Variable: <strong>y</strong></span><span class="text-slate-500">i32</span></div>
                    <div class="text-emerald-400 text-sm">Value: 42 (Bitwise Copy)</div>
                  </div>
                {:else}
                  <div class="p-4 rounded-lg border transition-all {memoryScenario === 'moved' ? 'bg-red-950/10 border-red-500/30 opacity-60' : 'bg-white/[0.02] border-white/10'} space-y-2">
                    <div class="flex justify-between text-slate-300">
                      <span>Variable: <strong class={memoryScenario === 'moved' ? 'line-through text-red-400' : ''}>s1</strong></span>
                      <span class="text-slate-500">String</span>
                    </div>
                    <div class="grid grid-cols-3 gap-2 text-[11px] text-slate-400 pt-1">
                      <div>ptr: <span class="text-slate-200">{memoryScenario === 'moved' ? 'INVALID' : '0x7FFF00'}</span></div>
                      <div>len: <span class="text-slate-200">4</span></div>
                      <div>cap: <span class="text-slate-200">4</span></div>
                    </div>
                    {#if memoryScenario === 'moved'}
                      <div class="text-[11px] text-red-400 pt-1">moved to s2 — pointer invalidated</div>
                    {/if}
                  </div>

                  {#if memoryScenario === "moved" || memoryScenario === "cloned"}
                    <div class="p-4 rounded-lg bg-white/[0.02] border {memoryScenario === 'moved' ? 'border-rust' : 'border-emerald-500/30'} space-y-2">
                      <div class="flex justify-between text-slate-300"><span>Variable: <strong>s2</strong></span><span class="text-slate-500">String</span></div>
                      <div class="grid grid-cols-3 gap-2 text-[11px] text-slate-400 pt-1">
                        <div>ptr: <span class="text-emerald-400">{memoryScenario === 'cloned' ? '0x8FFF00' : '0x7FFF00'}</span></div>
                        <div>len: <span class="text-slate-200">4</span></div>
                        <div>cap: <span class="text-slate-200">4</span></div>
                      </div>
                    </div>
                  {/if}
                {/if}
              </div>

              <!-- Heap Allocation Box -->
              <div class="p-5 rounded-xl bg-black/40 border border-white/[0.05] space-y-4 flex flex-col justify-between">
                <div>
                  <span class="text-slate-500 block text-[11px] mb-3">HEAP ALLOCATIONS (PHYSICAL RAM)</span>
                  
                  {#if memoryScenario === "copy"}
                    <div class="p-6 text-center text-slate-500 text-xs border border-dashed border-white/10 rounded-lg">
                      No heap allocation required. Primitives reside entirely on the CPU stack.
                    </div>
                  {:else}
                    <div class="space-y-3">
                      <div class="p-3.5 rounded-lg border border-white/10 bg-white/[0.02] flex items-center justify-between">
                        <div>
                          <span class="text-[10px] text-slate-500 block">ADDR: 0x7FFF00</span>
                          <span class="text-white tracking-widest text-sm font-bold">[ 'R', 'u', 's', 't' ]</span>
                        </div>
                        <span class="text-xs text-slate-400">Owner: <strong class="text-slate-200">{memoryScenario === 'moved' ? 's2' : 's1'}</strong></span>
                      </div>

                      {#if memoryScenario === "cloned"}
                        <div class="p-3.5 rounded-lg border border-emerald-500/30 bg-emerald-950/10 flex items-center justify-between">
                          <div>
                            <span class="text-[10px] text-emerald-500 block">ADDR: 0x8FFF00 (New Allocation)</span>
                            <span class="text-emerald-300 tracking-widest text-sm font-bold">[ 'R', 'u', 's', 't' ]</span>
                          </div>
                          <span class="text-xs text-slate-400">Owner: <strong class="text-emerald-400">s2</strong></span>
                        </div>
                      {/if}
                    </div>
                  {/if}
                </div>

                <div class="text-[11px] text-slate-500 pt-3 border-t border-white/[0.05]">
                  {#if memoryScenario === 'moved'}
                    Compile-time check: trying to pass `s1` to `println!` after move triggers <span class="text-red-400 font-mono">error[E0382]: borrow of moved value</span>.
                  {:else if memoryScenario === 'cloned'}
                    Cloning duplicates the entire heap buffer, doubling memory bandwidth usage.
                  {:else}
                    Single owner `s1` holds exclusive responsibility for freeing address `0x7FFF00` on scope exit.
                  {/if}
                </div>
              </div>
            </div>
          </div>

          <!-- Code Comparison -->
          <div class="p-6 rounded-xl bg-[#0d0e12] border border-white/[0.06] font-mono text-xs space-y-3">
            <div class="text-slate-400 flex justify-between border-b border-white/[0.06] pb-3">
              <span>src/main.rs</span>
              <span class="text-slate-300 font-medium">Ownership Rules Verification</span>
            </div>
            <pre class="text-slate-300 leading-relaxed overflow-x-auto"><code>fn main() &#123;
    let s1 = String::from("hello");
    let s2 = s1; <span class="text-slate-500">// Stack pointer moved to s2. s1 is marked uninitialized.</span>

    <span class="text-slate-500">// println!("&#123;s1&#125;"); // &lt;-- COMPILER ERROR: value used here after move</span>
    println!("s2 owns the heap buffer: &#123;s2&#125;");
&#125;</code></pre>
          </div>

        </section>
      {/if}

      <!-- ════════════════════════════════════════════════════════════════ -->
      <!-- TAB 02: Borrow Checker                                           -->
      <!-- ════════════════════════════════════════════════════════════════ -->
      {#if activeTab === 1}
        <section class="space-y-12 animate-fadeIn">
          
          <div class="space-y-4 max-w-3xl">
            <h2 class="text-2xl md:text-3xl font-bold text-white tracking-tight">
              The Borrow Checker & Reference Aliasing
            </h2>
            <p class="text-slate-400 leading-relaxed text-base">
              Borrowing allows code to inspect values without taking ownership by passing references (<code class="text-slate-300 font-mono">&T</code> or <code class="text-slate-300 font-mono">&mut T</code>). To eliminate data races and iterator invalidation at compile time, Rust enforces strict aliasing invariants: you may have <strong>any number of immutable references</strong> OR <strong>exactly one mutable reference</strong> to a resource at a given time, but never both simultaneously within the same scope.
            </p>
          </div>

          <!-- Borrow Matrix Sandbox -->
          <div class="p-8 rounded-2xl bg-[#0d0e12] border border-white/[0.08] space-y-8">
            <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 pb-6 border-b border-white/[0.06]">
              <div>
                <h3 class="font-medium text-white text-base">Reference Aliasing Engine</h3>
                <p class="text-xs text-slate-400 mt-1">Test reference allocations against compile-time borrowing invariants.</p>
              </div>
              <div class="flex gap-2 text-xs font-mono">
                <button
                  on:click={addImmutBorrow}
                  class="px-3.5 py-1.5 rounded bg-white/5 hover:bg-white/10 border border-white/10 text-slate-300 transition-colors"
                >
                  + Add &buffer (Shared Read)
                </button>
                <button
                  on:click={toggleMutBorrow}
                  class="px-3.5 py-1.5 rounded {mutActive ? 'bg-rust/20 border-rust text-rust' : 'bg-white/5 border-white/10 text-slate-300'} hover:bg-white/10 transition-colors"
                >
                  Toggle &mut buffer (Exclusive Write)
                </button>
                <button
                  on:click={resetBorrows}
                  class="px-3 py-1.5 rounded bg-white/5 hover:bg-white/10 border border-white/10 text-slate-400 transition-colors"
                >
                  Reset Scopes
                </button>
              </div>
            </div>

            <!-- Visual State -->
            <div class="grid grid-cols-1 sm:grid-cols-3 gap-4 font-mono text-xs">
              <div class="p-5 rounded-xl bg-black/40 border border-white/[0.05] space-y-2">
                <span class="text-slate-500 block text-[11px]">IMMUTABLE BORROWS (&T)</span>
                <div class="text-2xl font-mono font-medium text-slate-200">{immutCount} <span class="text-xs font-normal text-slate-500">active</span></div>
                <div class="text-[11px] text-slate-400">Shared read access guaranteed without mutation.</div>
              </div>

              <div class="p-5 rounded-xl bg-black/40 border border-white/[0.05] space-y-2">
                <span class="text-slate-500 block text-[11px]">MUTABLE BORROW (&mut T)</span>
                <div class="text-2xl font-mono font-medium {mutActive ? 'text-rust' : 'text-slate-500'}">
                  {mutActive ? "1 ACTIVE" : "0 ACTIVE"}
                </div>
                <div class="text-[11px] text-slate-400">Exclusive write access. Requires 0 immutable borrows.</div>
              </div>

              <div class="p-5 rounded-xl bg-black/40 border border-white/[0.05] space-y-2">
                <span class="text-slate-500 block text-[11px]">COMPILER STATUS</span>
                <div class="text-sm font-medium {borrowError ? 'text-red-400' : 'text-emerald-400'}">
                  {borrowError ? "COMPILE FAILED" : "BORROW CHECK PASSED"}
                </div>
                <div class="text-[11px] text-slate-500">Non-Lexical Lifetimes (NLL) verified.</div>
              </div>
            </div>

            {#if borrowError}
              <div class="p-4 rounded-xl bg-red-950/20 border border-red-500/30 text-red-300 font-mono text-xs leading-relaxed">
                <strong>Borrow Checker Error:</strong> {borrowError}
              </div>
            {/if}
          </div>

          <!-- Code Snippet -->
          <div class="p-6 rounded-xl bg-[#0d0e12] border border-white/[0.06] font-mono text-xs space-y-3">
            <div class="text-slate-400 flex justify-between border-b border-white/[0.06] pb-3">
              <span>src/main.rs</span>
              <span class="text-slate-300 font-medium">Non-Lexical Lifetimes (NLL)</span>
            </div>
            <pre class="text-slate-300 leading-relaxed overflow-x-auto"><code>let mut buffer = String::from("System Buffer");

let r1 = &buffer; // Shared immutable borrow starts
let r2 = &buffer;
println!("Read shared: &#123;r1&#125;, &#123;r2&#125;");
<span class="text-slate-500">// Under NLL, r1 and r2 are no longer used after this line; their lifetimes end.</span>

let r_mut = &mut buffer; <span class="text-slate-500">// Valid: No active immutable borrows overlap here.</span>
r_mut.push_str(" [MODIFIED]");</code></pre>
          </div>

        </section>
      {/if}

      <!-- ════════════════════════════════════════════════════════════════ -->
      <!-- TAB 03: Slice Views                                              -->
      <!-- ════════════════════════════════════════════════════════════════ -->
      {#if activeTab === 2}
        <section class="space-y-12 animate-fadeIn">
          
          <div class="space-y-4 max-w-3xl">
            <h2 class="text-2xl md:text-3xl font-bold text-white tracking-tight">
              Zero-Allocation Slice Views (<code class="text-slate-200 font-mono text-xl">&str</code> & <code class="text-slate-200 font-mono text-xl">&[T]</code>)
            </h2>
            <p class="text-slate-400 leading-relaxed text-base">
              Slices are references to a contiguous sequence of elements within a collection. Under the hood, a slice is a two-word "fat pointer" containing a memory address and a length (<code class="text-slate-300 font-mono">ptr</code>, <code class="text-slate-300 font-mono">len</code>). Slicing allows systems functions to inspect partial buffers or string prefixes without copying payloads or performing heap allocations.
            </p>
          </div>

          <!-- Slice Offset Sandbox -->
          <div class="p-8 rounded-2xl bg-[#0d0e12] border border-white/[0.08] space-y-8">
            <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 pb-6 border-b border-white/[0.06]">
              <div>
                <h3 class="font-medium text-white text-base">String Slice Inspector (&str)</h3>
                <p class="text-xs text-slate-400 mt-1">Adjust byte offsets to inspect zero-copy substring fat pointers.</p>
              </div>
              <div class="flex items-center gap-4 text-xs font-mono">
                <button
                  on:click={() => { sliceStart = 0; sliceEnd = 6; }}
                  class="px-3 py-1.5 rounded bg-white/5 hover:bg-white/10 border border-white/10 text-slate-300"
                >
                  [0..6] (Status)
                </button>
                <button
                  on:click={() => { sliceStart = 7; sliceEnd = 19; }}
                  class="px-3 py-1.5 rounded bg-white/5 hover:bg-white/10 border border-white/10 text-slate-300"
                >
                  [7..19] (CPU Metric)
                </button>
                <button
                  on:click={() => { sliceStart = 20; sliceEnd = 26; }}
                  class="px-3 py-1.5 rounded bg-white/5 hover:bg-white/10 border border-white/10 text-slate-300"
                >
                  [20..26] (Mem Metric)
                </button>
              </div>
            </div>

            <!-- Buffer Characters Grid -->
            <div class="space-y-4 font-mono text-xs">
              <span class="text-slate-500 block text-[11px]">HOST MEMORY BUFFER: `SYS_OK:CPU_LOAD_12%:MEM_OK`</span>
              
              <div class="flex flex-wrap gap-1.5 p-4 rounded-xl bg-black/40 border border-white/[0.05]">
                {#each rawString.split("") as char, idx}
                  <div class="w-8 h-10 rounded border flex flex-col items-center justify-center transition-all {idx >= sliceStart && idx < sliceEnd ? 'bg-rust/20 border-rust text-white font-bold scale-105' : 'bg-white/[0.02] border-white/10 text-slate-500'}">
                    <span class="text-sm">{char}</span>
                    <span class="text-[9px] text-slate-600">{idx}</span>
                  </div>
                {/each}
              </div>

              <!-- Fat Pointer Output -->
              <div class="grid grid-cols-1 sm:grid-cols-3 gap-4 pt-4">
                <div class="p-4 rounded-xl bg-black/40 border border-white/[0.05]">
                  <span class="text-slate-500 block text-[11px] mb-1">SLICE FAT POINTER</span>
                  <div class="text-slate-200 font-mono">ptr: <span class="text-emerald-400">0x7FFF00 + {sliceStart}</span></div>
                  <div class="text-slate-200 font-mono">len: <span class="text-slate-300">{sliceEnd - sliceStart}</span> bytes</div>
                </div>

                <div class="p-4 rounded-xl bg-black/40 border border-white/[0.05]">
                  <span class="text-slate-500 block text-[11px] mb-1">EVALUATED &str VALUE</span>
                  <div class="text-lg font-bold text-white tracking-wide">
                    "{rawString.slice(sliceStart, sliceEnd)}"
                  </div>
                </div>

                <div class="p-4 rounded-xl bg-black/40 border border-white/[0.05]">
                  <span class="text-slate-500 block text-[11px] mb-1">ALLOCATION COST</span>
                  <div class="text-emerald-400 font-medium">0 Bytes Allocated</div>
                  <div class="text-[11px] text-slate-500">No heap copying performed.</div>
                </div>
              </div>
            </div>
          </div>

          <!-- Code Snippet -->
          <div class="p-6 rounded-xl bg-[#0d0e12] border border-white/[0.06] font-mono text-xs space-y-3">
            <div class="text-slate-400 flex justify-between border-b border-white/[0.06] pb-3">
              <span>src/main.rs</span>
              <span class="text-slate-300 font-medium">Slice Fat Pointers</span>
            </div>
            <pre class="text-slate-300 leading-relaxed overflow-x-auto"><code>let telemetry = String::from("SYS_OK:CPU_LOAD_12%:MEM_OK");

<span class="text-slate-500">// Slice syntax generates a 16-byte fat pointer (8-byte address + 8-byte length)</span>
let status: &str = &telemetry[0..6]; 
let cpu_metric: &str = &telemetry[7..19];

println!("Status: &#123;status&#125;"); <span class="text-slate-500">// Prints "SYS_OK" without heap cloning</span></code></pre>
          </div>

        </section>
      {/if}

      <!-- ════════════════════════════════════════════════════════════════ -->
      <!-- TAB 04: RAII & Scopes                                            -->
      <!-- ════════════════════════════════════════════════════════════════ -->
      {#if activeTab === 3}
        <section class="space-y-12 animate-fadeIn">
          
          <div class="space-y-4 max-w-3xl">
            <h2 class="text-2xl md:text-3xl font-bold text-white tracking-tight">
              RAII & Deterministic Scope Unwinding
            </h2>
            <p class="text-slate-400 leading-relaxed text-base">
              Resource Acquisition Is Initialization (RAII) is Rust's core resource management pattern. When a variable is declared within a block scope `&#123; ... &#125;`, it acquires its resources. When execution exits the scope—whether through normal completion or panic unwinding—Rust automatically invokes the <code class="text-slate-300 font-mono">Drop::drop()</code> method on all owned variables in reverse order of initialization (LIFO).
            </p>
          </div>

          <!-- Scope Unwinding Simulator -->
          <div class="p-8 rounded-2xl bg-[#0d0e12] border border-white/[0.08] space-y-8">
            <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 pb-6 border-b border-white/[0.06]">
              <div>
                <h3 class="font-medium text-white text-base">Scope Unwinding Simulator</h3>
                <p class="text-xs text-slate-400 mt-1">Step through scope exit to observe LIFO resource deallocation.</p>
              </div>
              <div class="flex gap-2 text-xs font-mono">
                <button
                  on:click={() => scopeStep = 0}
                  class="px-3.5 py-1.5 rounded bg-white/5 hover:bg-white/10 border border-white/10 text-slate-300"
                >
                  1. Enter Block Scope
                </button>
                <button
                  on:click={() => scopeStep = 1}
                  class="px-3.5 py-1.5 rounded bg-white/5 hover:bg-white/10 border border-white/10 text-slate-300"
                >
                  2. Drop res3 (Buffer)
                </button>
                <button
                  on:click={() => scopeStep = 2}
                  class="px-3.5 py-1.5 rounded bg-white/5 hover:bg-white/10 border border-white/10 text-slate-300"
                >
                  3. Drop res2 (FileDesc)
                </button>
                <button
                  on:click={() => scopeStep = 3}
                  class="px-3.5 py-1.5 rounded bg-emerald-500/10 hover:bg-emerald-500/20 border border-emerald-500/20 text-emerald-300 font-medium"
                >
                  4. Scope Fully Unwound
                </button>
              </div>
            </div>

            <!-- Stack Frame Visualization -->
            <div class="grid grid-cols-1 md:grid-cols-3 gap-4 font-mono text-xs">
              {#each raiiStack as item, idx}
                {@const isDropped = (scopeStep === 1 && idx === 2) || (scopeStep === 2 && idx >= 1) || scopeStep === 3}
                <div class="p-5 rounded-xl border transition-all duration-300 {isDropped ? 'bg-red-950/10 border-red-500/20 opacity-50' : 'bg-black/40 border-white/[0.08]'} space-y-3">
                  <div class="flex justify-between items-center">
                    <span class="font-bold {isDropped ? 'line-through text-red-400' : 'text-white'}">{item.name}</span>
                    <span class="text-[10px] px-2 py-0.5 rounded {isDropped ? 'bg-red-500/10 text-red-400' : 'bg-emerald-500/10 text-emerald-400'}">
                      {isDropped ? 'DROPPED' : 'ACTIVE'}
                    </span>
                  </div>
                  <div class="text-slate-400 text-[11px]">Type: <strong class="text-slate-200">{item.type}</strong></div>
                  <div class="text-slate-500 text-[11px]">Addr: {item.addr}</div>
                  {#if isDropped}
                    <div class="text-[10px] text-red-400 pt-2 border-t border-red-500/20">
                      Drop::drop() executed. Memory & OS handles closed.
                    </div>
                  {/if}
                </div>
              {/each}
            </div>

            <div class="p-4 rounded-xl bg-white/[0.02] border border-white/[0.06] text-xs font-mono text-slate-400">
              {#if scopeStep === 0}
                Status: All 3 resources active inside curly brace block `&#123; ... &#125;`.
              {:else if scopeStep === 1}
                Status: Exiting scope. `res3` (last allocated) is dropped first following LIFO ordering.
              {:else if scopeStep === 2}
                Status: Unwinding continues. `res2` dropped second.
              {:else}
                Verified: Scope unwound completely. Zero memory leaks, dangling file descriptors, or open sockets.
              {/if}
            </div>
          </div>

          <!-- Code Snippet -->
          <div class="p-6 rounded-xl bg-[#0d0e12] border border-white/[0.06] font-mono text-xs space-y-3">
            <div class="text-slate-400 flex justify-between border-b border-white/[0.06] pb-3">
              <span>src/main.rs</span>
              <span class="text-slate-300 font-medium">RAII Drop Trait</span>
            </div>
            <pre class="text-slate-300 leading-relaxed overflow-x-auto"><code>&#123;
    let _res1 = ScopeTracker("Network Socket");  <span class="text-slate-500">// 1st allocated</span>
    let _res2 = ScopeTracker("File Descriptor"); <span class="text-slate-500">// 2nd allocated</span>
    let _res3 = ScopeTracker("Memory Buffer");   <span class="text-slate-500">// 3rd allocated</span>
    
    <span class="text-slate-500">// Do work...</span>
&#125; <span class="text-slate-500">// &lt;-- Scope ends here. Rust automatically calls Drop in LIFO order:</span>
  <span class="text-slate-500">//     1. _res3 dropped</span>
  <span class="text-slate-500">//     2. _res2 dropped</span>
  <span class="text-slate-500">//     3. _res1 dropped</span></code></pre>
          </div>

        </section>
      {/if}

    </main>
  </div>
</div>

<style>
  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(8px); }
    to { opacity: 1; transform: translateY(0); }
  }
  .animate-fadeIn {
    animation: fadeIn 0.3s cubic-bezier(0.16, 1, 0.3, 1) forwards;
  }
</style>
