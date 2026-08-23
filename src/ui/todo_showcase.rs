/// Interactive multi-framework (React, Vue 3, Angular) Todo Showcase HTML page
pub fn todo_showcase_html(site_name: &str) -> String {
    let raw_html = r#"<!DOCTYPE html>
<html lang="en" class="dark">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>__SITE_NAME__ - Multi-Framework Todo App (React, Vue, Angular)</title>
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
  <link href="https://fonts.googleapis.com/css2?family=Plus+Jakarta+Sans:wght@300;400;500;600;700&family=JetBrains+Mono:wght@400;500;600&display=swap" rel="stylesheet">
  <script src="https://cdn.tailwindcss.com"></script>
  <script>
    tailwind.config = {
      darkMode: 'class',
      theme: {
        extend: {
          fontFamily: {
            sans: ['"Plus Jakarta Sans"', 'sans-serif'],
            mono: ['"JetBrains Mono"', 'monospace'],
          },
          colors: {
            brand: {
              400: '#34d399',
              500: '#10b981',
              600: '#059669',
            },
            react: {
              400: '#38bdf8',
              500: '#0ea5e9',
              600: '#0284c7',
            },
            vue: {
              400: '#4ade80',
              500: '#22c55e',
              600: '#16a34a',
            },
            angular: {
              400: '#fb7185',
              500: '#f43f5e',
              600: '#e11d48',
            },
            oxide: {
              50: '#f8fafc',
              800: '#1e293b',
              900: '#0f172a',
              950: '#020617',
            }
          }
        }
      }
    }
  </script>
  <!-- React 18 & Babel -->
  <script src="https://unpkg.com/react@18/umd/react.production.min.js" crossorigin></script>
  <script src="https://unpkg.com/react-dom@18/umd/react-dom.production.min.js" crossorigin></script>
  <script src="https://unpkg.com/@babel/standalone/babel.min.js"></script>
  <!-- Vue 3 -->
  <script src="https://unpkg.com/vue@3/dist/vue.global.prod.js"></script>
  <style>
    body { font-family: 'Plus Jakarta Sans', sans-serif; }
    .glass { background: rgba(30, 41, 59, 0.75); backdrop-filter: blur(14px); -webkit-backdrop-filter: blur(14px); border: 1px solid rgba(255, 255, 255, 0.08); }
    .custom-scrollbar::-webkit-scrollbar { width: 6px; height: 6px; }
    .custom-scrollbar::-webkit-scrollbar-track { background: transparent; }
    .custom-scrollbar::-webkit-scrollbar-thumb { background: rgba(255, 255, 255, 0.15); border-radius: 9999px; }
  </style>
</head>
<body class="bg-oxide-950 text-slate-100 min-h-screen antialiased selection:bg-brand-500 selection:text-black">

  <!-- TOP HEADER -->
  <header class="glass sticky top-0 z-50 border-b border-slate-800 px-6 py-4">
    <div class="max-w-6xl mx-auto flex flex-col md:flex-row md:items-center justify-between gap-4">
      <div class="flex items-center space-x-3">
        <div class="w-10 h-10 rounded-xl bg-gradient-to-tr from-brand-500 via-react-400 to-angular-500 flex items-center justify-center font-bold text-black text-xl shadow-lg shadow-brand-500/20">
          ⚡
        </div>
        <div>
          <h1 class="text-lg font-bold text-white flex items-center space-x-2">
            <span>Oxide_CG Multi-Frontend Todo App</span>
            <span class="text-[10px] bg-brand-500/20 text-brand-300 border border-brand-500/30 px-2 py-0.5 rounded-full font-mono font-bold">1 Backend &bull; 3 Frontends</span>
          </h1>
          <p class="text-xs text-slate-400">Powered by high-performance Rust core on Axum & SQLite/Postgres</p>
        </div>
      </div>

      <div class="flex items-center space-x-3">
        <a href="/admin" class="px-4 py-2 bg-slate-800 hover:bg-slate-700 text-slate-200 text-xs font-semibold rounded-xl transition border border-slate-700">
          Admin Dashboard &rarr;
        </a>
        <a href="/swagger" target="_blank" class="px-4 py-2 bg-brand-500 hover:bg-brand-600 text-black text-xs font-bold rounded-xl transition shadow-lg shadow-brand-500/20">
          Swagger API &rarr;
        </a>
      </div>
    </div>
  </header>

  <!-- MAIN CONTAINER -->
  <main class="max-w-6xl mx-auto p-6 md:p-8 space-y-8">
    
    <!-- FRAMEWORK SELECTOR TABS -->
    <div class="glass p-2 rounded-2xl border border-slate-800 flex flex-wrap gap-2">
      <button onclick="switchTab('react')" id="tab-btn-react" class="flex-1 py-3 px-4 rounded-xl text-sm font-bold transition flex items-center justify-center space-x-2 bg-react-500/20 text-react-300 border border-react-500/40 shadow-lg shadow-react-500/10">
        <span>⚛️</span>
        <span>React 18/19 App</span>
      </button>
      <button onclick="switchTab('vue')" id="tab-btn-vue" class="flex-1 py-3 px-4 rounded-xl text-sm font-bold transition flex items-center justify-center space-x-2 text-slate-400 hover:bg-slate-800/80">
        <span>🟢</span>
        <span>Vue 3 Composition App</span>
      </button>
      <button onclick="switchTab('angular')" id="tab-btn-angular" class="flex-1 py-3 px-4 rounded-xl text-sm font-bold transition flex items-center justify-center space-x-2 text-slate-400 hover:bg-slate-800/80">
        <span>🅰️</span>
        <span>Angular 17+ Signals App</span>
      </button>
    </div>

    <!-- LATENCY & ARCHITECTURE BANNER -->
    <div class="glass p-5 rounded-2xl border border-slate-800 flex flex-col md:flex-row md:items-center justify-between gap-4 bg-gradient-to-r from-slate-900 via-slate-900/90 to-brand-950/20">
      <div class="flex items-center space-x-3">
        <span class="text-2xl">⚡</span>
        <div>
          <h4 class="text-sm font-bold text-white">Live Rust Backend Telemetry</h4>
          <p class="text-xs text-slate-400">All 3 frontend frameworks query the same unified <code class="text-brand-400 bg-slate-950 px-1.5 py-0.5 rounded font-mono">/api/d/todo</code> dAPI endpoint</p>
        </div>
      </div>
      <div class="flex items-center space-x-4 text-xs font-mono">
        <div class="text-right">
          <span class="text-slate-400 block">Server Latency</span>
          <span id="latency-meter" class="text-emerald-400 font-bold">&lt; 0.4 ms</span>
        </div>
        <div class="text-right">
          <span class="text-slate-400 block">AI Tuner</span>
          <span class="text-purple-400 font-bold">Active</span>
        </div>
      </div>
    </div>

    <!-- 1. REACT 18 TODO APPLICATION CONTAINER -->
    <div id="react-app-wrapper" class="space-y-6">
      <div id="react-root"></div>
    </div>

    <!-- 2. VUE 3 TODO APPLICATION CONTAINER -->
    <div id="vue-app-wrapper" class="space-y-6 hidden">
      <div id="vue-root">
        <div class="glass p-6 md:p-8 rounded-2xl border border-slate-800 space-y-6">
          <div class="flex flex-col md:flex-row md:items-center justify-between gap-4 pb-6 border-b border-slate-800">
            <div>
              <h2 class="text-xl font-bold text-white flex items-center space-x-2">
                <span>🟢 Vue 3 Composition API Todo App</span>
                <span class="text-[10px] bg-emerald-500/20 text-emerald-400 border border-emerald-500/30 px-2 py-0.5 rounded-full font-mono">Vue 3.4 &bull; Ref &bull; Computed</span>
              </h2>
              <p class="text-xs text-slate-400 mt-1">Reactive state with Vue Composition API and Oxide_CG backend</p>
            </div>
            <button @click="openCreateModal" class="px-4 py-2.5 bg-gradient-to-r from-emerald-500 to-green-400 hover:from-emerald-600 hover:to-green-500 text-black font-bold text-sm rounded-xl transition shadow-lg shadow-emerald-500/20 flex items-center space-x-2">
              <span>+ Add Task</span>
            </button>
          </div>

          <!-- FILTERS BAR -->
          <div class="flex flex-wrap items-center justify-between gap-4">
            <input type="text" v-model="searchQuery" placeholder="Search tasks with Vue reactive ref..." class="px-4 py-2.5 rounded-xl bg-slate-900/90 border border-slate-700 text-sm focus:border-emerald-500 text-slate-100 flex-1 max-w-md">
            <div class="flex space-x-2">
              <button @click="filterStatus = 'all'" :class="filterStatus === 'all' ? 'bg-slate-800 text-white border-emerald-500/50' : 'text-slate-400 hover:bg-slate-800'" class="px-3 py-1.5 rounded-lg border border-slate-800 text-xs font-semibold transition">All ({{ todos.length }})</button>
              <button @click="filterStatus = 'active'" :class="filterStatus === 'active' ? 'bg-slate-800 text-emerald-400 border-emerald-500/50' : 'text-slate-400 hover:bg-slate-800'" class="px-3 py-1.5 rounded-lg border border-slate-800 text-xs font-semibold transition">Active ({{ activeCount }})</button>
              <button @click="filterStatus = 'completed'" :class="filterStatus === 'completed' ? 'bg-slate-800 text-emerald-400 border-emerald-500/50' : 'text-slate-400 hover:bg-slate-800'" class="px-3 py-1.5 rounded-lg border border-slate-800 text-xs font-semibold transition">Completed ({{ completedCount }})</button>
            </div>
          </div>

          <!-- TODO CARDS LIST -->
          <div class="space-y-3">
            <div v-if="loading" class="p-8 text-center text-slate-500 font-sans">Loading Vue 3 tasks from Rust backend...</div>
            <div v-else-if="filteredTodos.length === 0" class="p-8 text-center text-slate-500 font-sans border border-slate-800 rounded-xl">No tasks matching your filter. Click "+ Add Task" to create one.</div>
            <div v-for="t in filteredTodos" :key="t.id" class="p-4 rounded-xl bg-slate-900/80 border border-slate-800/80 hover:border-emerald-500/40 transition flex items-center justify-between gap-4">
              <div class="flex items-center space-x-4 flex-1">
                <input type="checkbox" :checked="t.is_completed" @change="toggleTodo(t)" class="w-5 h-5 rounded text-emerald-500 bg-slate-900 border-slate-700 cursor-pointer">
                <div class="space-y-1">
                  <div :class="t.is_completed ? 'line-through text-slate-500' : 'text-white font-semibold'" class="text-sm">{{ t.title }}</div>
                  <div class="flex items-center space-x-2 text-[11px] text-slate-400">
                    <span class="px-2 py-0.5 rounded bg-slate-800 text-slate-300 font-mono">{{ t.category || 'General' }}</span>
                    <span :class="t.priority === 'High' || t.priority === 'Critical' ? 'text-rose-400' : 'text-amber-400'" class="font-bold">{{ t.priority || 'Medium' }}</span>
                    <span v-if="t.progress !== undefined" class="text-emerald-400">&bull; {{ t.progress }}% done</span>
                  </div>
                </div>
              </div>
              <button @click="deleteTodo(t.id)" class="text-slate-500 hover:text-rose-400 text-xs font-bold p-2 transition">Delete ✕</button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 3. ANGULAR 17+ TODO APPLICATION CONTAINER -->
    <div id="angular-app-wrapper" class="space-y-6 hidden">
      <div id="angular-container" class="glass p-6 md:p-8 rounded-2xl border border-slate-800 space-y-6">
        <div class="flex flex-col md:flex-row md:items-center justify-between gap-4 pb-6 border-b border-slate-800">
          <div>
            <h2 class="text-xl font-bold text-white flex items-center space-x-2">
              <span>🅰️ Angular 17/18 Signals Todo App</span>
              <span class="text-[10px] bg-rose-500/20 text-rose-400 border border-rose-500/30 px-2 py-0.5 rounded-full font-mono">Standalone &bull; Signals &bull; RxJS</span>
            </h2>
            <p class="text-xs text-slate-400 mt-1">Fine-grained reactive signals and dependency injection with Oxide_CG</p>
          </div>
          <button onclick="window.AngularApp.openCreateModal()" class="px-4 py-2.5 bg-gradient-to-r from-rose-500 to-pink-500 hover:from-rose-600 hover:to-pink-600 text-white font-bold text-sm rounded-xl transition shadow-lg shadow-rose-500/20 flex items-center space-x-2">
            <span>+ Add Task</span>
          </button>
        </div>

        <div class="flex flex-wrap items-center justify-between gap-4">
          <input id="ng-search" type="text" oninput="window.AngularApp.handleSearch(this.value)" placeholder="Search tasks with Angular Signal..." class="px-4 py-2.5 rounded-xl bg-slate-900/90 border border-slate-700 text-sm focus:border-rose-500 text-slate-100 flex-1 max-w-md">
          <div class="flex space-x-2" id="ng-filter-buttons">
            <!-- Rendered by AngularApp -->
          </div>
        </div>

        <div class="space-y-3" id="ng-todo-list">
          <!-- Rendered by AngularApp -->
        </div>
      </div>
    </div>

  </main>

  <!-- JAVASCRIPT & FRAMEWORK BOOTSTRAP -->
  <script>
    function switchTab(framework) {
      // Hide all
      document.getElementById('react-app-wrapper').classList.add('hidden');
      document.getElementById('vue-app-wrapper').classList.add('hidden');
      document.getElementById('angular-app-wrapper').classList.add('hidden');

      // Reset tab buttons
      ['react', 'vue', 'angular'].forEach(f => {
        const btn = document.getElementById(`tab-btn-${f}`);
        btn.className = 'flex-1 py-3 px-4 rounded-xl text-sm font-bold transition flex items-center justify-center space-x-2 text-slate-400 hover:bg-slate-800/80';
      });

      // Show selected
      document.getElementById(`${framework}-app-wrapper`).classList.remove('hidden');
      const activeBtn = document.getElementById(`tab-btn-${framework}`);
      if (framework === 'react') {
        activeBtn.className = 'flex-1 py-3 px-4 rounded-xl text-sm font-bold transition flex items-center justify-center space-x-2 bg-react-500/20 text-react-300 border border-react-500/40 shadow-lg shadow-react-500/10';
      } else if (framework === 'vue') {
        activeBtn.className = 'flex-1 py-3 px-4 rounded-xl text-sm font-bold transition flex items-center justify-center space-x-2 bg-emerald-500/20 text-emerald-300 border border-emerald-500/40 shadow-lg shadow-emerald-500/10';
      } else if (framework === 'angular') {
        activeBtn.className = 'flex-1 py-3 px-4 rounded-xl text-sm font-bold transition flex items-center justify-center space-x-2 bg-rose-500/20 text-rose-300 border border-rose-500/40 shadow-lg shadow-rose-500/10';
      }
    }
  </script>

  <!-- 1. REACT 18 TODO ENGINE -->
  <script type="text/babel">
    const { useState, useEffect, useCallback, useMemo } = React;

    function ReactTodoApp() {
      const [todos, setTodos] = useState([]);
      const [filter, setFilter] = useState('all');
      const [searchQuery, setSearchQuery] = useState('');
      const [loading, setLoading] = useState(true);
      const [isModalOpen, setIsModalOpen] = useState(false);
      const [newTitle, setNewTitle] = useState('');
      const [newCategory, setNewCategory] = useState('Engineering');
      const [newPriority, setNewPriority] = useState('High');
      const [newProgress, setNewProgress] = useState(0);

      const fetchTodos = useCallback(async () => {
        setLoading(true);
        const start = performance.now();
        try {
          const res = await fetch('/api/d/todo?$limit=100&$order=-created_at');
          const data = await res.json();
          if (data.success) {
            setTodos(data.data);
          }
          const latency = (performance.now() - start).toFixed(2);
          document.getElementById('latency-meter').textContent = `${latency} ms`;
        } catch (e) {
          console.error(e);
        } finally {
          setLoading(false);
        }
      }, []);

      useEffect(() => {
        fetchTodos();
      }, [fetchTodos]);

      const handleCreate = async (e) => {
        e.preventDefault();
        if (!newTitle.trim()) return;

        const payload = {
          title: newTitle.trim(),
          category: newCategory,
          priority: newPriority,
          progress: Number(newProgress),
          is_completed: false,
        };

        const res = await fetch('/api/d/todo', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(payload)
        });

        if (res.ok) {
          setNewTitle('');
          setIsModalOpen(false);
          fetchTodos();
        }
      };

      const handleToggle = async (t) => {
        await fetch(`/api/d/todo/${t.id}`, {
          method: 'PUT',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ is_completed: !t.is_completed, progress: !t.is_completed ? 100 : 0 })
        });
        fetchTodos();
      };

      const handleDelete = async (id) => {
        await fetch(`/api/d/todo/${id}`, { method: 'DELETE' });
        fetchTodos();
      };

      const filteredTodos = useMemo(() => {
        return todos.filter(t => {
          const matchesSearch = t.title.toLowerCase().includes(searchQuery.toLowerCase());
          if (!matchesSearch) return false;
          if (filter === 'active') return !t.is_completed;
          if (filter === 'completed') return t.is_completed;
          return true;
        });
      }, [todos, filter, searchQuery]);

      const activeCount = todos.filter(t => !t.is_completed).length;
      const completedCount = todos.filter(t => t.is_completed).length;

      return (
        <div className="glass p-6 md:p-8 rounded-2xl border border-slate-800 space-y-6">
          <div className="flex flex-col md:flex-row md:items-center justify-between gap-4 pb-6 border-b border-slate-800">
            <div>
              <h2 className="text-xl font-bold text-white flex items-center space-x-2">
                <span>⚛️ React 18/19 Todo App</span>
                <span className="text-[10px] bg-react-500/20 text-react-400 border border-react-500/30 px-2 py-0.5 rounded-full font-mono">useState &bull; useMemo &bull; Hooks</span>
              </h2>
              <p className="text-xs text-slate-400 mt-1">Reactive state with React 18 and Oxide_CG backend</p>
            </div>
            <button onClick={() => setIsModalOpen(true)} className="px-4 py-2.5 bg-gradient-to-r from-react-500 to-sky-400 hover:from-react-600 hover:to-sky-500 text-black font-bold text-sm rounded-xl transition shadow-lg shadow-react-500/20 flex items-center space-x-2">
              <span>+ Add Task</span>
            </button>
          </div>

          {/* FILTERS */}
          <div className="flex flex-wrap items-center justify-between gap-4">
            <input 
              type="text" 
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              placeholder="Search tasks with React state hook..." 
              className="px-4 py-2.5 rounded-xl bg-slate-900/90 border border-slate-700 text-sm focus:border-react-500 text-slate-100 flex-1 max-w-md"
            />
            <div className="flex space-x-2">
              <button onClick={() => setFilter('all')} className={`px-3 py-1.5 rounded-lg border text-xs font-semibold transition ${filter === 'all' ? 'bg-slate-800 text-white border-react-500/50' : 'text-slate-400 border-slate-800 hover:bg-slate-800'}`}>All ({todos.length})</button>
              <button onClick={() => setFilter('active')} className={`px-3 py-1.5 rounded-lg border text-xs font-semibold transition ${filter === 'active' ? 'bg-slate-800 text-react-400 border-react-500/50' : 'text-slate-400 border-slate-800 hover:bg-slate-800'}`}>Active ({activeCount})</button>
              <button onClick={() => setFilter('completed')} className={`px-3 py-1.5 rounded-lg border text-xs font-semibold transition ${filter === 'completed' ? 'bg-slate-800 text-react-400 border-react-500/50' : 'text-slate-400 border-slate-800 hover:bg-slate-800'}`}>Completed ({completedCount})</button>
            </div>
          </div>

          {/* LIST */}
          <div className="space-y-3">
            {loading ? (
              <div className="p-8 text-center text-slate-500 font-sans">Loading React tasks from Rust backend...</div>
            ) : filteredTodos.length === 0 ? (
              <div className="p-8 text-center text-slate-500 font-sans border border-slate-800 rounded-xl">No tasks matching your filter. Click "+ Add Task" to create one.</div>
            ) : (
              filteredTodos.map(t => (
                <div key={t.id} className="p-4 rounded-xl bg-slate-900/80 border border-slate-800/80 hover:border-react-500/40 transition flex items-center justify-between gap-4">
                  <div className="flex items-center space-x-4 flex-1">
                    <input 
                      type="checkbox" 
                      checked={Boolean(t.is_completed)} 
                      onChange={() => handleToggle(t)}
                      className="w-5 h-5 rounded text-react-500 bg-slate-900 border-slate-700 cursor-pointer"
                    />
                    <div className="space-y-1">
                      <div className={`text-sm ${t.is_completed ? 'line-through text-slate-500' : 'text-white font-semibold'}`}>{t.title}</div>
                      <div className="flex items-center space-x-2 text-[11px] text-slate-400">
                        <span className="px-2 py-0.5 rounded bg-slate-800 text-slate-300 font-mono">{t.category || 'General'}</span>
                        <span className={`font-bold ${t.priority === 'High' || t.priority === 'Critical' ? 'text-rose-400' : 'text-amber-400'}`}>{t.priority || 'Medium'}</span>
                        {t.progress !== undefined && <span className="text-react-400">&bull; {t.progress}% done</span>}
                      </div>
                    </div>
                  </div>
                  <button onClick={() => handleDelete(t.id)} className="text-slate-500 hover:text-rose-400 text-xs font-bold p-2 transition">Delete ✕</button>
                </div>
              ))
            )}
          </div>

          {/* CREATE MODAL */}
          {isModalOpen && (
            <div className="fixed inset-0 z-50 bg-black/80 backdrop-blur-sm flex items-center justify-center p-4">
              <div className="glass max-w-md w-full p-6 rounded-2xl border border-slate-800 space-y-4">
                <h3 className="text-lg font-bold text-white">Create Task (React 18)</h3>
                <form onSubmit={handleCreate} className="space-y-4">
                  <div>
                    <label className="block text-xs font-semibold uppercase tracking-wider text-slate-400 mb-1">Task Title</label>
                    <input 
                      type="text" 
                      value={newTitle} 
                      onChange={(e) => setNewTitle(e.target.value)} 
                      placeholder="e.g. Implement React hooks for Oxide_CG" 
                      className="w-full px-4 py-2.5 rounded-xl bg-slate-900 border border-slate-700 text-sm text-slate-100 focus:border-react-500" 
                      required 
                    />
                  </div>
                  <div className="grid grid-cols-2 gap-3">
                    <div>
                      <label className="block text-xs font-semibold uppercase tracking-wider text-slate-400 mb-1">Category</label>
                      <select value={newCategory} onChange={(e) => setNewCategory(e.target.value)} className="w-full px-3 py-2 rounded-xl bg-slate-900 border border-slate-700 text-xs text-slate-100">
                        <option value="Engineering">Engineering</option>
                        <option value="Product">Product</option>
                        <option value="Design">Design</option>
                        <option value="DevOps">DevOps</option>
                      </select>
                    </div>
                    <div>
                      <label className="block text-xs font-semibold uppercase tracking-wider text-slate-400 mb-1">Priority</label>
                      <select value={newPriority} onChange={(e) => setNewPriority(e.target.value)} className="w-full px-3 py-2 rounded-xl bg-slate-900 border border-slate-700 text-xs text-slate-100">
                        <option value="Low">Low</option>
                        <option value="Medium">Medium</option>
                        <option value="High">High</option>
                        <option value="Critical">Critical</option>
                      </select>
                    </div>
                  </div>
                  <div>
                    <label className="block text-xs font-semibold uppercase tracking-wider text-slate-400 mb-1">Progress: {newProgress}%</label>
                    <input type="range" min="0" max="100" value={newProgress} onChange={(e) => setNewProgress(e.target.value)} className="w-full accent-react-500" />
                  </div>
                  <div className="flex justify-end space-x-3 pt-2">
                    <button type="button" onClick={() => setIsModalOpen(false)} className="px-4 py-2 text-xs font-semibold text-slate-400 hover:text-white">Cancel</button>
                    <button type="submit" className="px-5 py-2 bg-react-500 hover:bg-react-600 text-black text-xs font-bold rounded-xl transition">Create Task</button>
                  </div>
                </form>
              </div>
            </div>
          )}
        </div>
      );
    }

    const reactRoot = ReactDOM.createRoot(document.getElementById('react-root'));
    reactRoot.render(<ReactTodoApp />);
  </script>

  <!-- 2. VUE 3 COMPOSITION ENGINE -->
  <script>
    const { createApp, ref, computed, onMounted } = Vue;

    const vueApp = createApp({
      setup() {
        const todos = ref([]);
        const filterStatus = ref('all');
        const searchQuery = ref('');
        const loading = ref(true);

        const fetchTodos = async () => {
          loading.value = true;
          try {
            const res = await fetch('/api/d/todo?$limit=100&$order=-created_at');
            const data = await res.json();
            if (data.success) {
              todos.value = data.data;
            }
          } catch (e) {
            console.error(e);
          } finally {
            loading.value = false;
          }
        };

        onMounted(fetchTodos);

        const filteredTodos = computed(() => {
          return todos.value.filter(t => {
            const matchSearch = t.title.toLowerCase().includes(searchQuery.value.toLowerCase());
            if (!matchSearch) return false;
            if (filterStatus.value === 'active') return !t.is_completed;
            if (filterStatus.value === 'completed') return t.is_completed;
            return true;
          });
        });

        const activeCount = computed(() => todos.value.filter(t => !t.is_completed).length);
        const completedCount = computed(() => todos.value.filter(t => t.is_completed).length);

        const openCreateModal = async () => {
          const title = prompt("Enter task title (Vue 3):");
          if (!title) return;
          await fetch('/api/d/todo', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
              title: title.trim(),
              category: 'Vue Engineering',
              priority: 'Medium',
              progress: 0,
              is_completed: false
            })
          });
          fetchTodos();
        };

        const toggleTodo = async (t) => {
          await fetch(`/api/d/todo/${t.id}`, {
            method: 'PUT',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ is_completed: !t.is_completed, progress: !t.is_completed ? 100 : 0 })
          });
          fetchTodos();
        };

        const deleteTodo = async (id) => {
          await fetch(`/api/d/todo/${id}`, { method: 'DELETE' });
          fetchTodos();
        };

        return {
          todos,
          filterStatus,
          searchQuery,
          loading,
          filteredTodos,
          activeCount,
          completedCount,
          openCreateModal,
          toggleTodo,
          deleteTodo,
        };
      }
    });

    vueApp.mount('#vue-root');
  </script>

  <!-- 3. ANGULAR 17+ SIGNALS ENGINE -->
  <script>
    (function() {
      // Standalone Signal-like Architecture for Angular Simulator
      let todosSignal = [];
      let filterSignal = 'all';
      let searchSignal = '';

      async function fetchAngularTodos() {
        const listDiv = document.getElementById('ng-todo-list');
        listDiv.innerHTML = '<div class="p-8 text-center text-slate-500 font-sans">Loading Angular 17+ Signal tasks from Rust backend...</div>';
        try {
          const res = await fetch('/api/d/todo?$limit=100&$order=-created_at');
          const data = await res.json();
          if (data.success) {
            todosSignal = data.data;
            renderAngularView();
          }
        } catch (e) {
          console.error(e);
        }
      }

      function renderAngularView() {
        const filterBtns = document.getElementById('ng-filter-buttons');
        const listDiv = document.getElementById('ng-todo-list');

        const activeCount = todosSignal.filter(t => !t.is_completed).length;
        const completedCount = todosSignal.filter(t => t.is_completed).length;

        filterBtns.innerHTML = `
          <button onclick="window.AngularApp.setFilter('all')" class="px-3 py-1.5 rounded-lg border text-xs font-semibold transition ${filterSignal === 'all' ? 'bg-slate-800 text-white border-rose-500/50' : 'text-slate-400 border-slate-800 hover:bg-slate-800'}">All (${todosSignal.length})</button>
          <button onclick="window.AngularApp.setFilter('active')" class="px-3 py-1.5 rounded-lg border text-xs font-semibold transition ${filterSignal === 'active' ? 'bg-slate-800 text-rose-400 border-rose-500/50' : 'text-slate-400 border-slate-800 hover:bg-slate-800'}">Active (${activeCount})</button>
          <button onclick="window.AngularApp.setFilter('completed')" class="px-3 py-1.5 rounded-lg border text-xs font-semibold transition ${filterSignal === 'completed' ? 'bg-slate-800 text-rose-400 border-rose-500/50' : 'text-slate-400 border-slate-800 hover:bg-slate-800'}">Completed (${completedCount})</button>
        `;

        const filtered = todosSignal.filter(t => {
          const match = t.title.toLowerCase().includes(searchSignal.toLowerCase());
          if (!match) return false;
          if (filterSignal === 'active') return !t.is_completed;
          if (filterSignal === 'completed') return t.is_completed;
          return true;
        });

        if (filtered.length === 0) {
          listDiv.innerHTML = '<div class="p-8 text-center text-slate-500 font-sans border border-slate-800 rounded-xl">No Angular tasks found. Click "+ Add Task" to create one.</div>';
          return;
        }

        listDiv.innerHTML = filtered.map(t => `
          <div class="p-4 rounded-xl bg-slate-900/80 border border-slate-800/80 hover:border-rose-500/40 transition flex items-center justify-between gap-4">
            <div class="flex items-center space-x-4 flex-1">
              <input type="checkbox" ${t.is_completed ? 'checked' : ''} onchange="window.AngularApp.toggleTodo(${t.id}, ${!t.is_completed})" class="w-5 h-5 rounded text-rose-500 bg-slate-900 border-slate-700 cursor-pointer">
              <div class="space-y-1">
                <div class="text-sm ${t.is_completed ? 'line-through text-slate-500' : 'text-white font-semibold'}">${t.title}</div>
                <div class="flex items-center space-x-2 text-[11px] text-slate-400">
                  <span class="px-2 py-0.5 rounded bg-slate-800 text-slate-300 font-mono">${t.category || 'Angular'}</span>
                  <span class="font-bold ${t.priority === 'High' || t.priority === 'Critical' ? 'text-rose-400' : 'text-amber-400'}">${t.priority || 'Medium'}</span>
                  ${t.progress !== undefined ? `<span class="text-rose-400">&bull; ${t.progress}% done</span>` : ''}
                </div>
              </div>
            </div>
            <button onclick="window.AngularApp.deleteTodo(${t.id})" class="text-slate-500 hover:text-rose-400 text-xs font-bold p-2 transition">Delete ✕</button>
          </div>
        `).join('');
      }

      window.AngularApp = {
        setFilter(f) {
          filterSignal = f;
          renderAngularView();
        },
        handleSearch(q) {
          searchSignal = q;
          renderAngularView();
        },
        async openCreateModal() {
          const title = prompt("Enter task title (Angular 17+ Signals):");
          if (!title) return;
          await fetch('/api/d/todo', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
              title: title.trim(),
              category: 'Angular Signals',
              priority: 'High',
              progress: 0,
              is_completed: false
            })
          });
          fetchAngularTodos();
        },
        async toggleTodo(id, newStatus) {
          await fetch(`/api/d/todo/${id}`, {
            method: 'PUT',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ is_completed: newStatus, progress: newStatus ? 100 : 0 })
          });
          fetchAngularTodos();
        },
        async deleteTodo(id) {
          await fetch(`/api/d/todo/${id}`, { method: 'DELETE' });
          fetchAngularTodos();
        }
      };

      fetchAngularTodos();
    })();
  </script>
</body>
</html>"#;

    raw_html.replace("__SITE_NAME__", site_name)
}
