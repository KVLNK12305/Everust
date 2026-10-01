<script lang="ts">
  import { onMount } from "svelte";
  import { 
    Activity, AlertTriangle, CheckCircle2, ChevronRight, Clock, 
    Cpu, Filter, Layers, Lock, Play, RefreshCw, ShieldAlert, Terminal, Zap 
  } from "lucide-svelte";

  // ─── Flight Recording Sample Data (matches lock-contention workload) ───
  interface EventItem {
    id: number;
    timestamp_offset_ms: number;
    thread_id: number;
    task_id: number;
    type: "TaskSpawn" | "TaskWake" | "TaskPoll" | "LockWait" | "LockAcquire" | "LockRelease" | "BlockingOperation" | "TaskComplete";
    details: string;
    duration_ms?: number;
    severity?: "critical" | "warning" | "info";
  }

  interface FindingItem {
    code: string;
    title: string;
    description: string;
    severity: "critical" | "warning" | "info";
    task_id: number;
    lock_id?: number;
    duration_ms: number;
    timestamp_ms: number;
  }

  const recordingMetadata = {
    file: "lock_contention.evr",
    service: "lock-contention-service",
    pid: 4912,
    magic: "EVR1",
    version: 1,
    eventsCount: 142,
    durationMs: 48.6,
    bufferMode: "Rolling (20,000 capacity)"
  };

  const findings: FindingItem[] = [
    {
      code: "EVERUST-L01",
      title: "Severe Mutex Contention",
      description: "Task #100 held Lock #1 for 6.20ms during greedy lock acquisition, causing Task #101 and #102 to stall in LockWait.",
      severity: "critical",
      task_id: 101,
      lock_id: 1,
      duration_ms: 6.2,
      timestamp_ms: 12.4
    },
    {
      code: "EVERUST-B01",
      title: "Synchronous Blocking Call on Worker Thread",
      description: "Task #103 executed 'sync_file_read_on_worker' lasting 8.50ms without yielding, violating async cooperative scheduling.",
      severity: "critical",
      task_id: 103,
      duration_ms: 8.5,
      timestamp_ms: 22.1
    },
    {
      code: "EVERUST-T02",
      title: "Task Starvation in Run Queue",
      description: "Task #102 waited 11.40ms between wake notification and poll execution due to worker thread saturation.",
      severity: "warning",
      task_id: 102,
      duration_ms: 11.4,
      timestamp_ms: 31.8
    }
  ];

  const rawEvents: EventItem[] = [
    { id: 1, timestamp_offset_ms: 0.1, thread_id: 1, task_id: 100, type: "TaskSpawn", details: "worker-task-0 spawned" },
    { id: 2, timestamp_offset_ms: 0.2, thread_id: 2, task_id: 101, type: "TaskSpawn", details: "worker-task-1 spawned" },
    { id: 3, timestamp_offset_ms: 0.4, thread_id: 3, task_id: 102, type: "TaskSpawn", details: "worker-task-2 spawned" },
    { id: 4, timestamp_offset_ms: 0.5, thread_id: 4, task_id: 103, type: "TaskSpawn", details: "worker-task-3 spawned" },
    { id: 5, timestamp_offset_ms: 1.2, thread_id: 1, task_id: 100, type: "LockWait", details: "Lock #1 requested" },
    { id: 6, timestamp_offset_ms: 1.3, thread_id: 1, task_id: 100, type: "LockAcquire", details: "Acquired Lock #1", duration_ms: 0.1 },
    { id: 7, timestamp_offset_ms: 1.4, thread_id: 2, task_id: 101, type: "LockWait", details: "Lock #1 requested (BLOCKED)", severity: "warning" },
    { id: 8, timestamp_offset_ms: 7.5, thread_id: 1, task_id: 100, type: "LockRelease", details: "Released Lock #1 after 6.2ms hold", duration_ms: 6.2, severity: "critical" },
    { id: 9, timestamp_offset_ms: 7.6, thread_id: 2, task_id: 101, type: "LockAcquire", details: "Acquired Lock #1 after 6.2ms wait", duration_ms: 6.2, severity: "critical" },
    { id: 10, timestamp_offset_ms: 14.2, thread_id: 3, task_id: 102, type: "TaskWake", details: "Waker triggered from channel" },
    { id: 11, timestamp_offset_ms: 22.1, thread_id: 4, task_id: 103, type: "BlockingOperation", details: "sync_file_read_on_worker", duration_ms: 8.5, severity: "critical" },
    { id: 12, timestamp_offset_ms: 25.6, thread_id: 3, task_id: 102, type: "TaskPoll", details: "Polled on worker 3 after 11.4ms starvation", duration_ms: 0.8, severity: "warning" },
    { id: 13, timestamp_offset_ms: 45.1, thread_id: 1, task_id: 100, type: "TaskComplete", details: "Completed in 45.0ms" },
    { id: 14, timestamp_offset_ms: 46.2, thread_id: 2, task_id: 101, type: "TaskComplete", details: "Completed in 46.0ms" },
    { id: 15, timestamp_offset_ms: 48.0, thread_id: 3, task_id: 102, type: "TaskComplete", details: "Completed in 47.6ms" }
  ];

  // ─── Filter & Selection State ───
  let selectedTaskId: number | null = 101;
  let selectedFinding: FindingItem | null = findings[0];
  let eventFilter: string = "ALL";
  let scrubPositionMs = 7.6;

  $: filteredEvents = rawEvents.filter(ev => {
    if (selectedTaskId && ev.task_id !== selectedTaskId && eventFilter === "TASK") return false;
    if (eventFilter === "LOCKS" && !ev.type.startsWith("Lock")) return false;
    if (eventFilter === "CRITICAL" && ev.severity !== "critical") return false;
    return true;
  });

  function selectFinding(f: FindingItem) {
    selectedFinding = f;
    selectedTaskId = f.task_id;
    scrubPositionMs = f.timestamp_ms;
  }
</script>

<svelte:head>
  <title>Investigation Workbench — Everust</title>
</svelte:head>

<div class="min-h-screen bg-[#07080a] text-slate-200 font-sans pb-24 pt-20">
  <div class="max-w-7xl mx-auto px-6 space-y-6">

    <!-- Top Header: Recording Metadata -->
    <header class="bg-[#0f1117] border border-white/10 rounded-2xl p-6 flex flex-col md:flex-row justify-between items-start md:items-center gap-4">
      <div class="space-y-1">
        <div class="flex items-center gap-3">
          <span class="px-2 py-0.5 rounded bg-rust/20 border border-rust/40 text-rust font-mono text-xs font-semibold">
            {recordingMetadata.magic} v{recordingMetadata.version}
          </span>
          <h1 class="text-xl font-bold tracking-tight text-white flex items-center gap-2">
            <span>{recordingMetadata.file}</span>
            <span class="text-xs font-normal text-slate-400 font-mono">({recordingMetadata.service})</span>
          </h1>
        </div>
        <p class="text-xs text-slate-400 font-mono">
          PID: {recordingMetadata.pid} • Duration: {recordingMetadata.durationMs}ms • Events: {recordingMetadata.eventsCount} • {recordingMetadata.bufferMode}
        </p>
      </div>

      <div class="flex items-center gap-3">
        <div class="px-3 py-1.5 rounded-lg bg-emerald-500/10 border border-emerald-500/20 text-emerald-400 font-mono text-xs flex items-center gap-2">
          <span class="w-2 h-2 rounded-full bg-emerald-400 animate-pulse"></span>
          RECORDING VERIFIED
        </div>
        <a 
          href="/experiments/lock-contention"
          class="px-3 py-1.5 rounded-lg bg-white/5 hover:bg-white/10 border border-white/10 text-xs font-mono text-slate-300 transition-colors flex items-center gap-1.5"
        >
          <Cpu size={14} />
          <span>Workload Model</span>
        </a>
      </div>
    </header>

    <!-- Deterministic Findings Panel -->
    <section class="space-y-3">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-2 text-xs font-mono tracking-widest text-slate-400 uppercase">
          <ShieldAlert size={14} class="text-rust" />
          <span>Deterministic Analyzer Findings ({findings.length})</span>
        </div>
        <span class="text-[11px] font-mono text-slate-500">Evaluated with everust-analyzer v0.1</span>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        {#each findings as finding}
          <button 
            type="button"
            on:click={() => selectFinding(finding)}
            class="p-4 rounded-xl text-left transition-all border {selectedFinding?.code === finding.code ? 'bg-rust/10 border-rust shadow-[0_0_20px_rgba(206,75,41,0.2)]' : 'bg-[#0f1117] border-white/5 hover:border-white/20'}"
          >
            <div class="flex items-center justify-between mb-2">
              <span class="font-mono text-xs font-bold {finding.severity === 'critical' ? 'text-red-400' : 'text-amber-400'}">
                {finding.code}
              </span>
              <span class="text-[10px] font-mono px-2 py-0.5 rounded bg-white/5 text-slate-400">
                +{finding.duration_ms}ms
              </span>
            </div>
            <div class="text-sm font-semibold text-white mb-1">{finding.title}</div>
            <p class="text-xs text-slate-400 line-clamp-2 leading-relaxed">{finding.description}</p>
          </button>
        {/each}
      </div>
    </section>

    <!-- Main Investigation Area: Timeline & Detail Grid -->
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">

      <!-- Left 2 Cols: Interactive Execution Timeline & Event Stream -->
      <div class="lg:col-span-2 space-y-6">
        
        <!-- Timeline Canvas / Swimlanes -->
        <div class="bg-[#0f1117] border border-white/10 rounded-2xl p-6 space-y-4">
          <div class="flex items-center justify-between">
            <div class="flex items-center gap-2">
              <Activity size={16} class="text-rust" />
              <h2 class="text-sm font-mono font-bold uppercase tracking-wider text-white">Execution Timeline</h2>
            </div>
            <div class="text-xs font-mono text-slate-400">
              Scrub: <span class="text-rust font-bold">{scrubPositionMs.toFixed(1)}ms</span> / 48.6ms
            </div>
          </div>

          <!-- Timeline Chart Graphic -->
          <div class="space-y-3 pt-2">
            <!-- Worker 0 -->
            <div class="space-y-1">
              <div class="flex justify-between text-[11px] font-mono text-slate-400">
                <span>Task #100 (Worker 0)</span>
                <span class="text-red-400 font-bold">Lock #1 Held (6.2ms)</span>
              </div>
              <div class="h-6 w-full bg-white/5 rounded-md relative overflow-hidden flex items-center">
                <div class="absolute left-[3%] w-[13%] h-4 rounded bg-cyan-500/80"></div>
                <div class="absolute left-[16%] w-[35%] h-4 rounded bg-red-500 border border-red-300" title="Greedy Lock Hold (6.2ms)"></div>
                <div class="absolute left-[51%] w-[45%] h-4 rounded bg-cyan-500/50"></div>
              </div>
            </div>

            <!-- Worker 1 -->
            <div class="space-y-1">
              <div class="flex justify-between text-[11px] font-mono text-slate-400">
                <span class="text-amber-300 font-bold">Task #101 (Worker 1) — STALLED</span>
                <span class="text-amber-400">LockWait (6.2ms)</span>
              </div>
              <div class="h-6 w-full bg-white/5 rounded-md relative overflow-hidden flex items-center">
                <div class="absolute left-[3%] w-[13%] h-4 rounded bg-cyan-500/80"></div>
                <div class="absolute left-[16%] w-[35%] h-4 rounded bg-amber-500/90 border border-amber-300 animate-pulse" title="Stalled in LockWait"></div>
                <div class="absolute left-[51%] w-[20%] h-4 rounded bg-cyan-500/90"></div>
              </div>
            </div>

            <!-- Worker 2 -->
            <div class="space-y-1">
              <div class="flex justify-between text-[11px] font-mono text-slate-400">
                <span>Task #102 (Worker 2)</span>
                <span class="text-purple-400">Starved in Queue (11.4ms)</span>
              </div>
              <div class="h-6 w-full bg-white/5 rounded-md relative overflow-hidden flex items-center">
                <div class="absolute left-[29%] w-[24%] h-4 rounded bg-purple-500/80 border border-purple-300" title="Starved waiting for thread"></div>
                <div class="absolute left-[53%] w-[30%] h-4 rounded bg-cyan-500/80"></div>
              </div>
            </div>

            <!-- Worker 3 -->
            <div class="space-y-1">
              <div class="flex justify-between text-[11px] font-mono text-slate-400">
                <span>Task #103 (Worker 3)</span>
                <span class="text-red-400">Blocking Op (8.5ms)</span>
              </div>
              <div class="h-6 w-full bg-white/5 rounded-md relative overflow-hidden flex items-center">
                <div class="absolute left-[45%] w-[18%] h-4 rounded bg-red-600 border border-red-400" title="Blocking synchronous file read"></div>
                <div class="absolute left-[63%] w-[30%] h-4 rounded bg-cyan-500/80"></div>
              </div>
            </div>
          </div>

          <!-- Legend -->
          <div class="pt-3 border-t border-white/5 flex flex-wrap gap-4 text-[11px] font-mono text-slate-400">
            <div class="flex items-center gap-1.5"><span class="w-3 h-3 rounded bg-cyan-500"></span> Active Poll</div>
            <div class="flex items-center gap-1.5"><span class="w-3 h-3 rounded bg-red-500"></span> Greedy Lock Hold / Blocking</div>
            <div class="flex items-center gap-1.5"><span class="w-3 h-3 rounded bg-amber-500"></span> LockWait Stall</div>
            <div class="flex items-center gap-1.5"><span class="w-3 h-3 rounded bg-purple-500"></span> Starvation Delay</div>
          </div>
        </div>

        <!-- Event Stream Table -->
        <div class="bg-[#0f1117] border border-white/10 rounded-2xl p-6 space-y-4">
          <div class="flex items-center justify-between">
            <div class="flex items-center gap-2">
              <Terminal size={16} class="text-rust" />
              <h2 class="text-sm font-mono font-bold uppercase tracking-wider text-white">Event Stream Log</h2>
            </div>
            
            <div class="flex gap-1 bg-white/5 p-1 rounded-lg text-[11px] font-mono">
              <button 
                on:click={() => eventFilter = "ALL"}
                class="px-2.5 py-1 rounded {eventFilter === 'ALL' ? 'bg-white text-black font-bold' : 'text-slate-400 hover:text-white'}"
              >ALL</button>
              <button 
                on:click={() => eventFilter = "LOCKS"}
                class="px-2.5 py-1 rounded {eventFilter === 'LOCKS' ? 'bg-white text-black font-bold' : 'text-slate-400 hover:text-white'}"
              >LOCKS</button>
              <button 
                on:click={() => eventFilter = "CRITICAL"}
                class="px-2.5 py-1 rounded {eventFilter === 'CRITICAL' ? 'bg-white text-black font-bold' : 'text-slate-400 hover:text-white'}"
              >CRITICAL</button>
            </div>
          </div>

          <div class="overflow-x-auto">
            <table class="w-full text-left font-mono text-xs">
              <thead class="text-slate-500 border-b border-white/5 uppercase text-[10px]">
                <tr>
                  <th class="py-2 px-3">#</th>
                  <th class="py-2 px-3">Timestamp</th>
                  <th class="py-2 px-3">Task ID</th>
                  <th class="py-2 px-3">Event Type</th>
                  <th class="py-2 px-3">Details</th>
                  <th class="py-2 px-3">Duration</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-white/5">
                {#each filteredEvents as ev}
                  <tr 
                    on:click={() => selectedTaskId = ev.task_id}
                    class="hover:bg-white/[0.04] cursor-pointer transition-colors {selectedTaskId === ev.task_id ? 'bg-white/[0.03]' : ''}"
                  >
                    <td class="py-2.5 px-3 text-slate-500">{ev.id}</td>
                    <td class="py-2.5 px-3 text-slate-400">+{ev.timestamp_offset_ms.toFixed(1)}ms</td>
                    <td class="py-2.5 px-3 font-bold {selectedTaskId === ev.task_id ? 'text-rust' : 'text-white'}">
                      #{ev.task_id}
                    </td>
                    <td class="py-2.5 px-3">
                      <span class="px-2 py-0.5 rounded text-[10px] font-semibold 
                        {ev.type.startsWith('Lock') ? 'bg-amber-500/20 text-amber-300 border border-amber-500/30' : 
                         ev.type === 'BlockingOperation' ? 'bg-red-500/20 text-red-300 border border-red-500/30' :
                         ev.type === 'TaskComplete' ? 'bg-emerald-500/20 text-emerald-300' :
                         'bg-cyan-500/10 text-cyan-300'}">
                        {ev.type}
                      </span>
                    </td>
                    <td class="py-2.5 px-3 text-slate-300">{ev.details}</td>
                    <td class="py-2.5 px-3 font-semibold {ev.severity === 'critical' ? 'text-red-400' : 'text-slate-400'}">
                      {ev.duration_ms ? `${ev.duration_ms.toFixed(1)}ms` : '—'}
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        </div>

      </div>

      <!-- Right 1 Col: Task Detail & Incident Remediation -->
      <div class="space-y-6">

        <!-- Selected Task Inspector -->
        <div class="bg-[#0f1117] border border-white/10 rounded-2xl p-6 space-y-4">
          <div class="flex items-center justify-between pb-3 border-b border-white/5">
            <div class="flex items-center gap-2">
              <Zap size={16} class="text-rust" />
              <h2 class="text-sm font-mono font-bold uppercase tracking-wider text-white">Task #{selectedTaskId} Inspector</h2>
            </div>
            <span class="px-2 py-0.5 rounded bg-emerald-500/20 text-emerald-400 font-mono text-[10px]">
              Completed
            </span>
          </div>

          <div class="space-y-3 font-mono text-xs">
            <div class="flex justify-between py-1 border-b border-white/5">
              <span class="text-slate-500">Task Name</span>
              <span class="text-white">worker-task-{selectedTaskId ? selectedTaskId - 100 : 0}</span>
            </div>
            <div class="flex justify-between py-1 border-b border-white/5">
              <span class="text-slate-500">Spawned At</span>
              <span class="text-slate-300">+0.2ms</span>
            </div>
            <div class="flex justify-between py-1 border-b border-white/5">
              <span class="text-slate-500">Total Lifetime</span>
              <span class="text-slate-300">46.0ms</span>
            </div>
            <div class="flex justify-between py-1 border-b border-white/5">
              <span class="text-slate-500">Time Blocked on Locks</span>
              <span class="text-amber-400 font-bold">6.2ms (13.5%)</span>
            </div>
            <div class="flex justify-between py-1 border-b border-white/5">
              <span class="text-slate-500">Executor Run Queue Latency</span>
              <span class="text-purple-400 font-bold">0.8ms</span>
            </div>
          </div>

          <!-- Automated Diagnosis Guidance -->
          <div class="p-4 rounded-xl bg-rust/10 border border-rust/30 space-y-2">
            <div class="text-xs font-mono font-bold text-rust flex items-center gap-1.5 uppercase">
              <AlertTriangle size={14} />
              <span>Recommended Remediation</span>
            </div>
            <p class="text-xs text-slate-300 leading-relaxed">
              Task #{selectedTaskId} stalled waiting for Lock #1 held by Task #100. Consider using 
              <code class="text-amber-300 bg-black/40 px-1 py-0.5 rounded">parking_lot::FairMutex</code>
              or scoping lock guards to minimal critical sections to eliminate greedy thread starvation.
            </p>
          </div>
        </div>

        <!-- System Recorder Status -->
        <div class="bg-[#0f1117] border border-white/10 rounded-2xl p-6 space-y-4">
          <div class="flex items-center gap-2">
            <Clock size={16} class="text-rust" />
            <h2 class="text-sm font-mono font-bold uppercase tracking-wider text-white">Recorder Invariants</h2>
          </div>
          
          <ul class="text-xs font-mono space-y-2 text-slate-400">
            <li class="flex items-center gap-2">
              <CheckCircle2 size={14} class="text-emerald-400" />
              <span>Zero allocations in fast-path event emission</span>
            </li>
            <li class="flex items-center gap-2">
              <CheckCircle2 size={14} class="text-emerald-400" />
              <span>Nanosecond resolution monotonic timestamps</span>
            </li>
            <li class="flex items-center gap-2">
              <CheckCircle2 size={14} class="text-emerald-400" />
              <span>Deterministic diagnostic rules (no LLMs)</span>
            </li>
          </ul>
        </div>

      </div>

    </div>

  </div>
</div>
