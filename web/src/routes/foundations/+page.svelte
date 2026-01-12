<script>
  import { onMount } from 'svelte';
  import { gsap } from 'gsap';
  import { ArrowRight, BookOpen } from 'lucide-svelte';

  const topics = [
    { title: "Ownership & Borrowing", desc: "The soul of Rust. Stack vs Heap.", slug: "ownership" },
    { title: "Smart Pointers", desc: "Box, Rc, Arc, and RefCell deep dive.", slug: "smart-pointers" },
    { title: "Concurrency", desc: "Fearless concurrency with Threads & Channels.", slug: "concurrency" },
    { title: "Unsafe Rust", desc: "Breaking the rules to build abstractions.", slug: "unsafe" },
  ];

  onMount(() => {
    gsap.from(".topic-row", {
      x: -20,
      opacity: 0,
      stagger: 0.1,
      duration: 0.5
    });
  });
</script>

<div class="max-w-4xl mx-auto py-20">
  <header class="mb-16 border-b border-white/10 pb-8">
    <div class="flex items-center gap-3 text-rust mb-4">
      <BookOpen size={20} />
      <span class="font-mono text-sm tracking-widest uppercase">Knowledge Base</span>
    </div>
    <h1 class="text-5xl font-bold text-white mb-4">Foundations</h1>
    <p class="text-xl text-slate-400">
      Core concepts implemented from scratch. No frameworks, just `cargo new --lib`.
    </p>
  </header>

  <div class="grid gap-px bg-white/10 border border-white/10 rounded-lg overflow-hidden">
    {#each topics as topic}
      <a href="/foundations/{topic.slug}" class="topic-row group relative bg-[#0a0a0a] p-8 hover:bg-white/[0.02] transition-colors">
        <div class="flex items-center justify-between relative z-10">
          <div>
            <h3 class="text-2xl font-bold text-white mb-2 group-hover:text-rust transition-colors">{topic.title}</h3>
            <p class="text-slate-400 group-hover:text-slate-300">{topic.desc}</p>
          </div>
          <div class="text-white/20 group-hover:text-rust group-hover:translate-x-2 transition-all duration-300">
            <ArrowRight size={24} />
          </div>
        </div>
      </a>
    {/each}
  </div>
</div>