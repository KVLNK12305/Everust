<script lang="ts">
  import { onMount } from "svelte";
  import { gsap } from "gsap";
  import { ArrowLeft, Check, ShieldCheck, AlertCircle, Code, Layers, Sliders, Box, Cpu } from "lucide-svelte";

  // ─── Interactive State: Making Invalid States Unrepresentable ───
  let representationMode: "loose" | "strict" = "strict";
  let looseVersion: "v4" | "v6" = "v4";
  let looseV4Address = "192.168.1.1";
  let looseV6Address = "fe80::1"; // Contradiction possible!
  let enableContradiction = false;

  // ─── Interactive State: Tagged Union Memory Layout ───
  let selectedVariant: "v4" | "v6" = "v4";

  // ─── Interactive State: Struct Update Moves ───
  let structScenario: "valid_clone" | "invalid_move" = "valid_clone";
</script>

<svelte:head>
  <title>Type System & Invariants — Everust Foundations</title>
</svelte:head>

<div class="min-h-screen bg-[#07080a] text-slate-200 font-sans pb-24 pt-20">
  <div class="max-w-6xl mx-auto px-6 space-y-10">

    <!-- Header Navigation -->
    <header class="border-b border-white/10 pb-6 flex items-center justify-between">
      <div class="flex items-center gap-4">
        <a 
          href="/foundations"
          class="p-2 rounded-lg bg-white/5 hover:bg-white/10 text-slate-400 hover:text-white transition-colors"
        >
          <ArrowLeft size={16} />
        </a>
        <div>
          <span class="font-mono text-[10px] text-rust uppercase tracking-widest block">Module 02 • Foundations</span>
          <h1 class="text-2xl font-bold text-white tracking-tight">Type System & Memory Invariants</h1>
        </div>
      </div>
      <div class="px-3 py-1 rounded bg-white/5 border border-white/10 font-mono text-xs text-slate-400">
        foundations/structs_enums_regex
      </div>
    </header>

    <!-- Lab 1: Making Invalid States Unrepresentable -->
    <section class="bg-[#0f1117] border border-white/10 rounded-2xl p-8 space-y-6">
      <div class="flex items-start justify-between flex-wrap gap-4">
        <div>
          <div class="flex items-center gap-2 text-xs font-mono text-rust uppercase tracking-wider mb-1">
            <ShieldCheck size={14} />
            <span>Core Principle</span>
          </div>
          <h2 class="text-xl font-bold text-white">Make Invalid States Unrepresentable</h2>
          <p class="text-xs text-slate-400 max-w-2xl leading-relaxed mt-1">
            Move correctness guarantees from runtime validation checks into the compile-time type system.
          </p>
        </div>

        <div class="flex gap-2 bg-white/5 p-1 rounded-lg text-xs font-mono">
          <button 
            on:click={() => representationMode = "loose"}
            class="px-3 py-1.5 rounded transition-colors {representationMode === 'loose' ? 'bg-amber-500/20 text-amber-300 font-bold border border-amber-500/30' : 'text-slate-400 hover:text-white'}"
          >
            Loose (Struct + Enum)
          </button>
          <button 
            on:click={() => representationMode = "strict"}
            class="px-3 py-1.5 rounded transition-colors {representationMode === 'strict' ? 'bg-emerald-500/20 text-emerald-300 font-bold border border-emerald-500/30' : 'text-slate-400 hover:text-white'}"
          >
            Strict (Algebraic Enum)
          </button>
        </div>
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 items-stretch">
        <!-- Interactive Model Visualizer -->
        <div class="p-6 rounded-xl bg-black/40 border border-white/5 space-y-4 flex flex-col justify-between">
          <div class="space-y-4">
            <div class="text-xs font-mono uppercase text-slate-500 tracking-wider">State Model Inspection</div>
            
            {#if representationMode === "loose"}
              <div class="space-y-3 font-mono text-xs">
                <div class="p-4 rounded-lg bg-amber-500/5 border border-amber-500/20 space-y-2">
                  <div class="flex justify-between text-slate-400">
                    <span>Kind Tag:</span>
                    <span class="text-amber-400 font-bold">{looseVersion.toUpperCase()}</span>
                  </div>
                  <div class="flex justify-between text-slate-400">
                    <span>v4_addr:</span>
                    <span class="text-white">"{looseV4Address}"</span>
                  </div>
                  <div class="flex justify-between text-slate-400">
                    <span>v6_addr:</span>
                    <span class="{enableContradiction ? 'text-red-400 font-bold' : 'text-slate-500'}">
                      "{enableContradiction ? looseV6Address : 'None'}"
                    </span>
                  </div>
                </div>

                <div class="pt-2 flex items-center justify-between">
                  <span class="text-slate-400 text-xs font-sans">Simulate Mismatched State:</span>
                  <button 
                    on:click={() => enableContradiction = !enableContradiction}
                    class="px-3 py-1 rounded border text-xs font-mono transition-colors {enableContradiction ? 'bg-red-500/20 border-red-500/40 text-red-300' : 'bg-white/5 border-white/10 text-slate-300'}"
                  >
                    {enableContradiction ? "Contradiction Active!" : "Inject Invalid State"}
                  </button>
                </div>

                {#if enableContradiction}
                  <div class="p-3 rounded bg-red-500/10 border border-red-500/30 text-red-400 text-xs font-sans flex items-start gap-2">
                    <AlertCircle size={16} class="shrink-0 mt-0.5" />
                    <span><strong>Runtime Bug:</strong> Struct kind is set to V4, but contains a populated V6 address field! Logic must manually validate every read.</span>
                  </div>
                {/if}
              </div>
            {:else}
              <div class="space-y-3 font-mono text-xs">
                <div class="p-4 rounded-lg bg-emerald-500/5 border border-emerald-500/20 space-y-3">
                  <div class="text-emerald-400 font-bold">enum IpAddr &#123; V4([u8; 4]), V6([u16; 8]) &#125;</div>
                  
                  <div class="flex gap-2">
                    <button 
                      on:click={() => selectedVariant = "v4"}
                      class="px-3 py-1 rounded border text-xs {selectedVariant === 'v4' ? 'bg-emerald-500/20 border-emerald-500/40 text-emerald-300' : 'bg-white/5 border-white/10 text-slate-400'}"
                    >
                      IpAddr::V4([127, 0, 0, 1])
                    </button>
                    <button 
                      on:click={() => selectedVariant = "v6"}
                      class="px-3 py-1 rounded border text-xs {selectedVariant === 'v6' ? 'bg-emerald-500/20 border-emerald-500/40 text-emerald-300' : 'bg-white/5 border-white/10 text-slate-400'}"
                    >
                      IpAddr::V6([0xfe80, ...])
                    </button>
                  </div>

                  <div class="text-xs text-slate-300 font-sans pt-2">
                    Selected Variant: <span class="font-mono text-white font-bold">{selectedVariant.toUpperCase()}</span>.
                    Mutually exclusive by definition. Contradictory states are <strong>unrepresentable in memory</strong>.
                  </div>
                </div>

                <div class="p-3 rounded bg-emerald-500/10 border border-emerald-500/30 text-emerald-400 text-xs font-sans flex items-start gap-2">
                  <Check size={16} class="shrink-0 mt-0.5" />
                  <span><strong>Zero Validation Bugs:</strong> Compiler guarantees that if you match on <code>IpAddr::V4</code>, you only have access to 4 bytes.</span>
                </div>
              </div>
            {/if}
          </div>
        </div>

        <!-- Rust Code Comparison -->
        <div class="p-6 rounded-xl bg-black/60 border border-white/10 font-mono text-xs overflow-x-auto space-y-2">
          <div class="text-slate-500 pb-2 border-b border-white/5 flex justify-between">
            <span>Rust Implementation</span>
            <span class="text-rust">{representationMode === 'strict' ? 'Idiomatic Safe' : 'Anti-pattern'}</span>
          </div>

          {#if representationMode === "loose"}
            <pre class="text-slate-300 leading-relaxed"><span class="text-slate-500">// ❌ Two independent fields that can contradict</span>
<span class="text-amber-400">enum</span> IpKind &#123; V4, V6 &#125;

<span class="text-amber-400">struct</span> IpAddr &#123;
    kind: IpKind,
    v4_address: <span class="text-cyan-400">Option</span>&lt;String&gt;,
    v6_address: <span class="text-cyan-400">Option</span>&lt;String&gt;,
&#125;

<span class="text-slate-500">// Caller can construct:</span>
<span class="text-amber-400">let</span> bug = IpAddr &#123;
    kind: IpKind::V4,
    v4_address: <span class="text-cyan-400">None</span>,
    v6_address: <span class="text-cyan-400">Some</span>(<span class="text-emerald-300">"::1"</span>.into()), <span class="text-red-400">// CONTRADICTION!</span>
&#125;;</pre>
          {:else}
            <pre class="text-slate-300 leading-relaxed"><span class="text-slate-500">// ✅ Algebraic Data Type: Tagged Union</span>
<span class="text-amber-400">enum</span> IpAddr &#123;
    V4([u8; <span class="text-cyan-400">4</span>]),
    V6([u16; <span class="text-cyan-400">8</span>]),
&#125;

<span class="text-slate-500">// Exhaustive match guarantees correctness:</span>
<span class="text-amber-400">match</span> addr &#123;
    IpAddr::V4(octets) =&gt; route_v4(octets),
    IpAddr::V6(segments) =&gt; route_v6(segments),
&#125;
<span class="text-slate-500">// Mismatched tags and data cannot compile.</span></pre>
          {/if}
        </div>
      </div>
    </section>

    <!-- Lab 2: Struct Update Syntax & Move Semantics -->
    <section class="bg-[#0f1117] border border-white/10 rounded-2xl p-8 space-y-6">
      <div>
        <div class="flex items-center gap-2 text-xs font-mono text-rust uppercase tracking-wider mb-1">
          <Layers size={14} />
          <span>Memory & Move Semantics</span>
        </div>
        <h2 class="text-xl font-bold text-white">Struct Update Syntax: Moves vs Copies</h2>
        <p class="text-xs text-slate-400 max-w-2xl leading-relaxed mt-1">
          When constructing structs using <code>..base</code>, fields with the <code>Copy</code> trait are duplicated, while heap-allocated fields (like <code>String</code>) move ownership and invalidate the original variable!
        </p>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div class="p-6 rounded-xl bg-black/40 border border-white/5 space-y-4 font-mono text-xs">
          <div class="flex justify-between items-center text-slate-400 pb-2 border-b border-white/5">
            <span>struct User &#123; active: bool, uname: String, email: String &#125;</span>
          </div>

          <div class="space-y-2 text-slate-300">
            <div>let user1 = User &#123; active: true, uname: "Kushal", email: "old@example.com" &#125;;</div>
            <div class="pt-2 text-slate-500">// What happens to user1 when user2 is instantiated?</div>
          </div>

          <div class="flex gap-2 pt-2">
            <button 
              on:click={() => structScenario = "invalid_move"}
              class="px-3 py-1.5 rounded border text-xs {structScenario === 'invalid_move' ? 'bg-red-500/20 border-red-500/40 text-red-300 font-bold' : 'bg-white/5 border-white/10 text-slate-400'}"
            >
              Move Without Override
            </button>
            <button 
              on:click={() => structScenario = "valid_clone"}
              class="px-3 py-1.5 rounded border text-xs {structScenario === 'valid_clone' ? 'bg-emerald-500/20 border-emerald-500/40 text-emerald-300 font-bold' : 'bg-white/5 border-white/10 text-slate-400'}"
            >
              Explicit Clone / Override
            </button>
          </div>

          {#if structScenario === "invalid_move"}
            <div class="p-3 rounded bg-red-500/10 border border-red-500/30 text-red-400 text-xs font-sans">
              <strong>Compiler Error [E0382]:</strong> <code>user1.email</code> was moved into <code>user2</code>. You cannot access <code>user1</code> afterward because its heap pointer was transferred!
            </div>
          {:else}
            <div class="p-3 rounded bg-emerald-500/10 border border-emerald-500/30 text-emerald-400 text-xs font-sans">
              <strong>Safe Ownership:</strong> By either explicitly cloning the string fields or providing new values, both <code>user1</code> and <code>user2</code> remain independently valid owners.
            </div>
          {/if}
        </div>

        <div class="p-6 rounded-xl bg-black/60 border border-white/10 font-mono text-xs overflow-x-auto">
          <div class="text-slate-500 pb-2 border-b border-white/5">Compiler Diagnostics</div>
          {#if structScenario === "invalid_move"}
            <pre class="text-red-400 leading-relaxed">let user2 = User &#123;
    uname: String::from("NewName"),
    ..user1 <span class="text-slate-500">// &lt;-- email moves here!</span>
&#125;;

println!("&#123;&#125;", user1.email);
<span class="text-slate-400">^^^^^^^^^^^^^^^^^^^^^^^^^^</span>
<span class="text-red-300">error[E0382]: borrow of moved value: `user1.email`</span></pre>
          {:else}
            <pre class="text-emerald-400 leading-relaxed">let user2 = User &#123;
    uname: user1.uname.clone(),
    email: user1.email.clone(),
    ..user1 <span class="text-slate-500">// &lt;-- active (bool) implements Copy</span>
&#125;;

<span class="text-slate-300">// ✅ Both user1 and user2 valid!</span>
println!("user1: &#123;&#125;, user2: &#123;&#125;", user1.email, user2.email);</pre>
          {/if}
        </div>
      </div>
    </section>

  </div>
</div>
