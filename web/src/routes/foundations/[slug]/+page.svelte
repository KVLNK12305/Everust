<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/stores";
  import { gsap } from "gsap";
  import { ArrowLeft, RefreshCw, Sparkles, Image as ImageIcon } from "lucide-svelte";

  // ─── Module Metadata Mapping ───
  $: slug = $page.params.slug || "unknown";

  const moduleDirectory: Record<string, { num: string; title: string; sub: string }> = {
    "smart-pointers": {
      num: "02",
      title: "Smart Pointers & Abstractions",
      sub: "Manual memory management via Box, Rc, Arc, and RefCell."
    },
    "traits": {
      num: "03",
      title: "Traits & Zero-Cost Generics",
      sub: "Building shared behavior and bounded polymorphism."
    },
    "concurrency": {
      num: "04",
      title: "Fearless Concurrency",
      sub: "Threads, message passing, and shared state without data races."
    },
    "unsafe": {
      num: "05",
      title: "Unsafe Rust & Bare Metal",
      sub: "Stepping outside the compiler's guarantees to build systems abstractions."
    }
  };

  $: info = moduleDirectory[slug] || {
    num: "0X",
    title: "Advanced Systems Module",
    sub: "This module is in active development in the core repository."
  };

  // ─── Curated Meme Images ───
  const memeImages = [
    {
      url: "https://i.imgflip.com/4/1c1uej.jpg",
      title: "",
      caption: "NEW STUFF COMING SOON... STAY TUNED!"
    },
    {
      url: "https://i.imgflip.com/1g8my4.jpg",
      title: "Two Buttons Dilemma",
      caption: "Me deciding whether to `.clone()` everything or fight the borrow checker for 3 hours."
    },
    {
      url: "https://i.imgflip.com/26am.jpg",
      title: "Ancient Aliens",
      caption: "Why is the daemon blazingly fast with zero memory leaks? ... RUST."
    },
    {
      url: "https://i.imgflip.com/1bij.jpg",
      title: "One Does Not Simply",
      caption: "One does not simply write concurrent systems without data races."
    },
    {
      url: "https://i.imgflip.com/4t0m5.jpg",
      title: "Disaster Girl",
      caption: "Me watching the legacy C++ service segfault while our new Rust daemon runs smoothly."
    }
  ];

  let currentMemeIdx = 0;
  let imageError = false;

  function nextMeme() {
    imageError = false;
    currentMemeIdx = (currentMemeIdx + 1) % memeImages.length;
  }

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
  
  <!-- Sticky Header -->
  <header class="border-b border-white/[0.06] bg-[#090a0d]/90 backdrop-blur-md sticky top-0 z-40">
    <div class="max-w-5xl mx-auto px-6 h-16 flex items-center justify-between">
      <div class="flex items-center gap-6">
        <a href="/foundations" class="inline-flex items-center gap-2 text-xs font-mono text-slate-400 hover:text-white transition-colors no-underline">
          <ArrowLeft size={14} />
          <span>Foundations</span>
        </a>
        <div class="h-4 w-px bg-white/10"></div>
        <span class="text-xs font-mono text-slate-400">Module {info.num} • {info.title}</span>
      </div>
      <div class="flex items-center gap-3 text-xs font-mono text-slate-400">
        <span class="px-2.5 py-1 rounded-md bg-rust/10 border border-rust/30 text-rust font-medium">Coming Soon</span>
      </div>
    </div>
  </header>

  <div class="max-w-3xl mx-auto px-6 pt-16">
    
    <!-- Hero Title Section -->
    <div class="fade-in-up opacity-0 translate-y-3 mb-12 text-center space-y-4">
      <div class="inline-block px-3 py-1 rounded-md bg-white/[0.04] border border-white/10 text-xs font-mono text-slate-400">
        MODULE {info.num} • UNDER CONSTRUCTION
      </div>
      <h1 class="text-4xl md:text-5xl font-bold text-white tracking-tight leading-tight">
        {info.title}
      </h1>
      <p class="text-base md:text-lg text-slate-400 max-w-xl mx-auto font-light leading-relaxed">
        {info.sub}
      </p>
    </div>

    <!-- Centered Meme Image Card -->
    <div class="fade-in-up opacity-0 translate-y-3 p-6 md:p-8 rounded-2xl bg-[#0d0e12] border border-white/[0.08] space-y-6 flex flex-col items-center shadow-2xl">
      
      <div class="w-full flex items-center justify-between pb-4 border-b border-white/[0.06] text-xs font-mono text-slate-400">
        <span class="flex items-center gap-2 text-rust">
          <Sparkles size={14} />
          <span>{memeImages[currentMemeIdx].title}</span>
        </span>
        <span>#{currentMemeIdx + 1} of {memeImages.length}</span>
      </div>

      <!-- Meme Picture Container -->
      <div class="relative rounded-xl overflow-hidden border border-white/10 bg-black/60 max-w-lg w-full flex items-center justify-center min-h-[320px]">
        {#if !imageError}
          <img
            src={memeImages[currentMemeIdx].url}
            alt={memeImages[currentMemeIdx].title}
            on:error={() => imageError = true}
            class="max-h-[420px] w-auto object-contain rounded-lg shadow-lg transition-all duration-300"
          />
        {:else}
          <!-- Fallback if CDN is blocked by browser/adblocker -->
          <div class="p-12 text-center space-y-4 max-w-sm">
            <ImageIcon size={48} class="mx-auto text-slate-600" />
            <div class="text-lg font-bold text-white">NEW STUFF COMING SOON...</div>
            <div class="text-xs font-mono text-rust tracking-widest uppercase">STAY TUNED!</div>
            <p class="text-xs text-slate-500">
              (Leonardo DiCaprio raises his champagne glass in toast to safe systems programming)
            </p>
          </div>
        {/if}
      </div>

      <!-- Caption & Controls -->
      <div class="w-full text-center space-y-4 pt-2">
        <p class="text-base md:text-lg font-medium text-slate-200 tracking-wide italic">
          "{memeImages[currentMemeIdx].caption}"
        </p>

        <div class="pt-4 border-t border-white/[0.06] flex items-center justify-center">
          <button
            on:click={nextMeme}
            class="px-5 py-2.5 rounded-xl bg-white/5 hover:bg-white/10 border border-white/10 text-white font-medium text-xs font-mono flex items-center gap-2 transition-all hover:border-white/20 hover:scale-[1.02]"
          >
            <RefreshCw size={14} class="text-rust" />
            <span>Next Meme Image</span>
          </button>
        </div>
      </div>

    </div>

  </div>
</div>
