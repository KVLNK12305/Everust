<script lang="ts">
  import { onMount } from "svelte";
  import { gsap } from "gsap";
  import { TextPlugin } from "gsap/TextPlugin";
  import { Cpu, Zap, ShieldAlert, Activity, ArrowRight, Terminal, AlertTriangle } from "lucide-svelte";
  
  gsap.registerPlugin(TextPlugin);

  // Define data as a constant to ensure it loads
  const experiments = [
    { 
      id: "01", 
      title: "Lock Contention", 
      status: "ACTIVE", 
      desc: "Visualizing mutex starvation under high-load concurrent writes.",
      icon: Activity,
      color: "text-emerald-400",
      border: "hover:border-emerald-500/50",
      bg: "hover:bg-emerald-500/10"
    },
    { 
      id: "02", 
      title: "Unsafe Boundaries", 
      status: "WARNING", 
      desc: "Deliberately breaking memory safety to test FFI boundaries.",
      icon: ShieldAlert,
      color: "text-red-400",
      border: "hover:border-red-500/50",
      bg: "hover:bg-red-500/10"
    },
    { 
      id: "03", 
      title: "Async Runtime", 
      status: "IDLE", 
      desc: "Building a custom executor to understand waker logic.",
      icon: Zap,
      color: "text-yellow-400",
      border: "hover:border-yellow-500/50",
      bg: "hover:bg-yellow-500/10"
    },
    { 
      id: "04", 
      title: "Cache Locality", 
      status: "STABLE", 
      desc: "Benchmarking SoA vs AoS layouts in real-time.",
      icon: Cpu,
      color: "text-blue-400",
      border: "hover:border-blue-500/50",
      bg: "hover:bg-blue-500/10"
    }
  ];

  onMount(() => {
    // We use .to() from opacity-0 instead of .from() to avoid visibility locking
    const tl = gsap.timeline();

    tl.to(".hero-anim", {
      y: 0,
      opacity: 1,
      stagger: 0.1,
      duration: 0.8,
      ease: "power3.out"
    })
    .to(".card-anim", {
      y: 0,
      opacity: 1,
      stagger: 0.1,
      duration: 0.6,
      ease: "power2.out"
    }, "-=0.4");

    // Blinking cursor effect
    gsap.to(".terminal-cursor", {
      opacity: 0,
      repeat: -1,
      yoyo: true,
      duration: 0.8,
      ease: "steps(1)"
    });
  });
</script>

<div class="min-h-screen bg-[#0a0a0a] text-slate-200 px-6 py-24 font-sans selection:bg-rust selection:text-white overflow-x-hidden">
  
  <div class="fixed inset-0 pointer-events-none z-0">
    <div class="absolute top-0 left-1/4 w-[500px] h-[500px] bg-rust/5 rounded-full blur-[120px]"></div>
    <div class="absolute bottom-0 right-1/4 w-[500px] h-[500px] bg-blue-500/5 rounded-full blur-[120px]"></div>
    <div class="absolute inset-0 bg-[linear-gradient(rgba(255,255,255,0.02)_1px,transparent_1px),linear-gradient(90deg,rgba(255,255,255,0.02)_1px,transparent_1px)] bg-[size:4rem_4rem] [mask-image:radial-gradient(ellipse_60%_50%_at_50%_0%,#000_70%,transparent_100%)]"></div>
  </div>

  <div class="relative z-10 max-w-6xl mx-auto">
    
    <header class="mb-20 max-w-3xl">
      <div class="hero-anim opacity-0 translate-y-4 mb-6">
        <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full border border-rust/30 bg-rust/10 text-rust text-xs font-mono tracking-widest uppercase">
          <span class="w-1.5 h-1.5 rounded-full bg-rust animate-pulse"></span>
          Live Environment
        </div>
      </div>
      
      <h1 class="hero-anim opacity-0 translate-y-4 text-5xl md:text-7xl font-bold tracking-tighter text-white mb-6">
        System 
        <span class="text-transparent bg-clip-text bg-gradient-to-r from-[#b7410e] to-orange-500">
          Experiments
        </span>
      </h1>
      
      <p class="hero-anim opacity-0 translate-y-4 text-xl text-slate-400 leading-relaxed max-w-2xl">
        Systems reveal nothing willingly. They must be <span class="text-white">stressed</span>, <span class="text-white">isolated</span>, and <span class="text-white">observed</span>.
        This is the playground for unsafe invariants and failure boundaries.
      </p>
    </header>

    <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-24">
      {#each experiments as exp (exp.id)}
        <a href={exp.id === "01" ? "/experiments/lock-contention" : "#"} class="card-anim opacity-0 translate-y-4 group relative bg-white/5 border border-white/10 rounded-xl p-8 transition-all duration-300 hover:border-white/20 hover:bg-white/[0.07] hover:-translate-y-1 block no-underline text-inherit">
          
          <div class="flex justify-between items-start mb-8">
            <div class="p-3 bg-white/5 rounded-lg text-slate-300 group-hover:text-white transition-colors">
              <svelte:component this={exp.icon} size={28} />
            </div>
            <span class="font-mono text-[10px] tracking-widest {exp.color} border border-white/10 px-2 py-1 rounded bg-black/50">
              {exp.status}
            </span>
          </div>

          <div class="space-y-3">
            <span class="text-xs font-mono text-slate-500">EXP-{exp.id}</span>
            <h3 class="text-2xl font-bold text-white group-hover:text-rust transition-colors">{exp.title}</h3>
            <p class="text-slate-400 leading-relaxed text-sm">
              {exp.desc}
            </p>
          </div>

          <div class="mt-8 pt-6 border-t border-white/5 flex items-center justify-between opacity-60 group-hover:opacity-100 transition-opacity">
            <span class="font-mono text-xs text-slate-500 group-hover:text-white transition-colors">Initialize Sequence</span>
            <ArrowRight size={16} class="transform group-hover:translate-x-1 transition-transform" />
          </div>
        </a>
      {/each}
    </div>

    <div class="hero-anim opacity-0 translate-y-4 border border-yellow-500/20 bg-yellow-500/5 rounded-lg p-6 relative overflow-hidden group mb-12">
        <div class="absolute inset-0 bg-gradient-to-r from-transparent via-yellow-500/5 to-transparent -translate-x-full group-hover:animate-[shimmer_2s_infinite]"></div>
        <div class="flex items-start gap-4 relative z-10">
            <div class="p-2 bg-yellow-500/10 rounded text-yellow-500 mt-1">
                <AlertTriangle size={20} />
            </div>
            <div class="w-full">
                <h4 class="text-yellow-500 font-mono text-sm font-bold uppercase tracking-widest mb-2">Development Advisory</h4>
                <p class="text-slate-400 text-sm leading-relaxed max-w-2xl">This sector is under active compilation. Heavy construction ahead. Please maintain safety protocols.</p>
                <div class="mt-4 font-mono text-xs text-yellow-500/90 bg-black/40 w-full p-4 rounded border border-yellow-500/20">
                    <span class="text-slate-500">$</span> run_protocol --mode=wip<br>
                    <span class="text-emerald-500">>></span> executing: <span class="text-white font-bold">ochitsuke()</span> ... [CALM_DOWN_INITIATED]
                </div>
            </div>
        </div>
    </div>

    <div class="hero-anim opacity-0 translate-y-4 border-t border-white/10 pt-8">
      <div class="font-mono text-xs text-slate-500 flex flex-col md:flex-row justify-between gap-4">
        <div class="flex items-center gap-2">
          <Terminal size={14} />
          <span>daemon@everust:~$ tail -f /var/log/sys_experiments</span>
        </div>
        <div class="flex gap-8 opacity-50">
          <span>MEM: 14%</span>
          <span>CPU: <span class="text-rust">IDLE</span></span>
          <span>UPTIME: 42m 12s<span class="terminal-cursor">_</span></span>
        </div>
      </div>
    </div>

  </div>
</div>