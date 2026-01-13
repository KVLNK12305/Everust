<script lang="ts">
  import { onMount } from 'svelte';
  import { gsap } from 'gsap';
  // FIX 1: 'box' -> 'Box' (Icons must be PascalCase)
  import { ArrowRight, BookOpen, Shield, Box, Link2, Lock, Zap } from 'lucide-svelte';

  const topics = [
    { 
      id: "f1", 
      title: "Ownership & Borrowing", 
      desc: "The soul of Rust. Understanding move semantics and the borrow checker.", 
      slug: "ownership", 
      icon: Shield 
    },
    { 
      id: "f2", 
      title: "Smart Pointers", 
      desc: "Manual memory management via Box, Rc, Arc, and RefCell.", 
      slug: "smart-pointers", 
      icon: Box // FIX 1: Updated usage
    },
    { 
      id: "f3", 
      title: "Traits & Generics", 
      desc: "Building shared behavior and zero-cost abstractions.", 
      slug: "traits", 
      icon: Link2 
    },
    { 
      id: "f4", 
      title: "Concurrency Basics", 
      desc: "Threads, message passing, and shared state without fear.", 
      slug: "concurrency", 
      icon: Zap 
    },
    { 
      id: "f5", 
      title: "Unsafe Foundations", 
      desc: "Stepping outside the compiler's guarantees to build abstractions.", 
      slug: "unsafe", 
      icon: Lock 
    },
  ];

  onMount(() => {
    const tl = gsap.timeline();

    // 1. Draw the central spine line
    tl.to(".spine-line", {
      height: "100%",
      duration: 1.5,
      ease: "power4.inOut"
    })
    // 2. Animate the armor plates in from sides
    .to(".armor-plate", {
      opacity: 1,
      x: 0,
      rotationY: 0, // Animate to flat 0 or keep slight angle if preferred
      stagger: 0.15,
      duration: 0.8,
      ease: "back.out(1.2)"
    }, "-=1");
  });
</script>

<div class="min-h-screen bg-[#0a0a0a] text-slate-200 px-6 py-32 font-sans overflow-x-hidden">

  <div class="fixed inset-0 pointer-events-none z-0">
      <div class="absolute top-0 left-1/2 -translate-x-1/2 w-[600px] h-[600px] bg-rust/10 rounded-full blur-[150px]"></div>
       <div class="absolute inset-0 bg-[linear-gradient(rgba(255,255,255,0.02)_1px,transparent_1px),linear-gradient(90deg,rgba(255,255,255,0.02)_1px,transparent_1px)] bg-[size:3rem_3rem] [mask-image:radial-gradient(ellipse_60%_50%_at_50%_0%,#000_70%,transparent_100%)]"></div>
  </div>

  <div class="relative z-10 max-w-5xl mx-auto">

    <header class="mb-24 text-center">
      <div class="inline-flex items-center gap-3 text-rust mb-6 opacity-80">
        <BookOpen size={20} />
        <span class="font-mono text-sm tracking-[0.2em] uppercase">Core Knowledge Base</span>
      </div>
      <h1 class="text-6xl md:text-7xl font-bold text-white mb-6 tracking-tight">
        Foundations
      </h1>
      <p class="text-xl text-slate-400 max-w-2xl mx-auto leading-relaxed">
        The bedrock of the system. Pure Rust concepts implemented from scratch, without frameworks or abstractions.
      </p>
    </header>

    <div class="relative flex flex-col gap-12 py-10">
      
      <div class="spine-line absolute left-1/2 top-0 bottom-0 w-px bg-gradient-to-b from-transparent via-rust/50 to-transparent -translate-x-1/2 h-0 z-0 hidden md:block"></div>

      {#each topics as topic, i (topic.id)}
        <a href="/foundations/{topic.slug}" 
           class="armor-plate group relative w-full md:w-[85%] p-8 md:p-10 bg-[#0a0a0a] border border-white/10 rounded-2xl overflow-hidden transition-all duration-500 hover:-translate-y-2 hover:bg-white/[0.03] hover:shadow-[0_0_30px_rgba(183,65,14,0.1)] z-10 opacity-0 
           {i % 2 === 0 
             ? 'md:mr-auto md:-translate-x-12 md:border-l-4 md:border-l-rust/60 md:rounded-tl-none' 
             : 'md:ml-auto md:translate-x-12 md:border-r-4 md:border-r-rust/60 md:rounded-tr-none'}"
        >
          
          <div class="hidden md:block absolute top-1/2 -translate-y-1/2 w-4 h-4 bg-[#0a0a0a] border-2 border-rust rounded-full z-20
            {i % 2 === 0 ? '-right-[26px]' : '-left-[26px]'} group-hover:bg-rust group-hover:shadow-[0_0_10px_var(--color-rust)] transition-all"></div>

          <div class="flex items-center justify-between relative z-10">
            <div class="flex items-start gap-6">
              <div class="hidden md:flex h-14 w-14 shrink-0 items-center justify-center rounded-xl bg-rust/10 text-rust border border-rust/20 group-hover:scale-110 group-hover:bg-rust/20 transition-all duration-300">
                <svelte:component this={topic.icon} size={26} />
              </div>
              
              <div>
                 <span class="font-mono text-[10px] text-rust/60 tracking-widest uppercase mb-2 block">Module 0{i + 1}</span>
                <h3 class="text-2xl md:text-3xl font-bold text-white mb-3 group-hover:text-rust transition-colors">{topic.title}</h3>
                <p class="text-base md:text-lg text-slate-400 leading-relaxed group-hover:text-slate-300 max-w-xl">{topic.desc}</p>
              </div>
            </div>

            <div class="text-white/20 group-hover:text-rust group-hover:translate-x-3 transition-all duration-300">
              <ArrowRight size={28} />
            </div>
          </div>

          <div class="absolute inset-0 opacity-[0.03] bg-[linear-gradient(45deg,transparent_25%,rgba(255,255,255,0.05)_50%,transparent_75%,transparent_100%)] bg-[length:20px_20px] pointer-events-none"></div>
        </a>
      {/each}
    </div>

  </div>
</div>