<script lang="ts">
  import { page } from '$app/stores';
  import { Github, Terminal, Cpu, Activity, Home } from 'lucide-svelte';

  // Helper to check if a route is active
  $: isActive = (path: string) => $page.url.pathname.startsWith(path);

  const navItems = [
    { label: 'Foundations', path: '/foundations', icon: Terminal },
    { label: 'Experiments', path: '/experiments', icon: Cpu },
    { label: 'Benches', path: '/benches', icon: Activity }
  ];
</script>

<nav class="fixed top-0 left-0 w-full z-50 border-b border-white/5 bg-[#0a0a0a]/80 backdrop-blur-md">
  <div class="container mx-auto px-6 h-20 flex items-center justify-between">
    
    <a href="/" class="group flex items-center gap-3">
      <div class="relative flex h-10 w-10 items-center justify-center rounded-xl bg-white/5 border border-white/10 transition-all duration-300 group-hover:bg-rust/20 group-hover:border-rust/50">
        <span class="text-xl">🦀</span>
        <div class="absolute inset-0 bg-rust/20 blur-xl rounded-full opacity-0 group-hover:opacity-100 transition-opacity"></div>
      </div>
      
      <div class="flex flex-col">
        <span class="font-bold text-lg tracking-tight text-white group-hover:text-rust transition-colors">
          EVERUST
        </span>
        <span class="text-[10px] font-mono text-slate-500 uppercase tracking-widest">
          Systems Learning
        </span>
      </div>
    </a>

    <div class="hidden md:flex items-center gap-1 bg-white/5 p-1 rounded-full border border-white/5">
      {#each navItems as item}
        <a 
          href={item.path} 
          class="relative px-5 py-2 rounded-full text-sm font-medium transition-all duration-300 flex items-center gap-2
          {isActive(item.path) 
            ? 'text-black bg-white shadow-[0_0_20px_rgba(255,255,255,0.3)]' 
            : 'text-slate-400 hover:text-white hover:bg-white/5'}"
        >
          <svelte:component this={item.icon} size={16} />
          {item.label}
        </a>
      {/each}
    </div>

    <div class="flex items-center gap-6">
      
      <div class="hidden lg:flex items-center gap-2 text-[10px] font-mono text-emerald-500 bg-emerald-500/10 px-3 py-1 rounded-md border border-emerald-500/20">
        <span class="relative flex h-2 w-2">
          <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
          <span class="relative inline-flex rounded-full h-2 w-2 bg-emerald-500"></span>
        </span>
        DAEMON: ACTIVE
      </div>

      <a 
        href="https://github.com/yourusername/everust" 
        target="_blank" 
        class="text-slate-400 hover:text-white transition-colors"
      >
        <Github size={20} />
      </a>
    </div>

  </div>
</nav>