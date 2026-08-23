pub fn admin_react_spa_html(site_name: &str) -> String {
    let raw_html = r#"<!DOCTYPE html>
<html lang="en" class="dark">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>__SITE_NAME__ - Oxide_CG (React & AI Tuner)</title>
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
              50: '#ecfdf5',
              100: '#d1fae5',
              400: '#34d399',
              500: '#10b981',
              600: '#059669',
              700: '#047857',
            },
            react: {
              400: '#38bdf8',
              500: '#0ea5e9',
              600: '#0284c7',
            },
            ai: {
              400: '#c084fc',
              500: '#a855f7',
              600: '#9333ea',
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
  <!-- React 18 & Babel Standalone -->
  <script src="https://unpkg.com/react@18/umd/react.production.min.js" crossorigin></script>
  <script src="https://unpkg.com/react-dom@18/umd/react-dom.production.min.js" crossorigin></script>
  <script src="https://unpkg.com/@babel/standalone/babel.min.js"></script>
  <style>
    body { font-family: 'Plus Jakarta Sans', sans-serif; }
    .glass { background: rgba(30, 41, 59, 0.75); backdrop-filter: blur(14px); -webkit-backdrop-filter: blur(14px); border: 1px solid rgba(255, 255, 255, 0.08); }
    .custom-scrollbar::-webkit-scrollbar { width: 6px; height: 6px; }
    .custom-scrollbar::-webkit-scrollbar-track { background: transparent; }
    .custom-scrollbar::-webkit-scrollbar-thumb { background: rgba(255, 255, 255, 0.15); border-radius: 9999px; }
    .custom-scrollbar::-webkit-scrollbar-thumb:hover { background: rgba(255, 255, 255, 0.25); }
  </style>
</head>
<body class="bg-oxide-950 text-slate-100 min-h-screen antialiased selection:bg-brand-500 selection:text-black">
  <div id="root"></div>

  <!-- REACT APPLICATION ROOT -->
  <script type="text/babel">
    const { useState, useEffect, useCallback, useMemo, useRef } = React;

    const SITE_NAME = "__SITE_NAME__";

    // API Helper
    async function apiRequest(path, options = {}) {
      options.headers = options.headers || {};
      options.headers['Content-Type'] = 'application/json';
      const res = await fetch(path, options);
      if (res.status === 401 && path !== '/api/auth/login') {
        throw new Error('UNAUTHORIZED');
      }
      return res.json();
    }

    // MAIN ROOT REACT APP COMPONENT
    function App() {
      const [user, setUser] = useState(null);
      const [schemas, setSchemas] = useState([]);
      const [currentTab, setCurrentTab] = useState('dashboard');
      const [currentModel, setCurrentModel] = useState(null);
      const [toast, setToast] = useState(null);
      const [pendingCount, setPendingCount] = useState(0);

      const showToast = useCallback((msg, isError = false) => {
        setToast({ msg, isError });
        setTimeout(() => setToast(null), 3500);
      }, []);

      const checkAuth = useCallback(async () => {
        try {
          const res = await apiRequest('/api/auth/me');
          if (res.success && res.user) {
            setUser(res.user);
            loadSchemas();
            loadApprovalsCount();
          } else {
            setUser(null);
          }
        } catch (e) {
          setUser(null);
        }
      }, []);

      const loadSchemas = useCallback(async () => {
        try {
          const res = await apiRequest('/api/d/schema');
          if (res.success) {
            setSchemas(res.schemas);
          }
        } catch (e) {
          console.error(e);
        }
      }, []);

      const loadApprovalsCount = useCallback(async () => {
        try {
          const res = await apiRequest('/api/d/approvals');
          if (res.success) {
            setPendingCount(res.data.length);
          }
        } catch (e) {}
      }, []);

      useEffect(() => {
        checkAuth();
      }, [checkAuth]);

      const handleLogout = async () => {
        await apiRequest('/api/auth/logout', { method: 'POST' });
        setUser(null);
        showToast('Logged out successfully');
      };

      if (!user) {
        return <AuthModal onLoginSuccess={(u) => { setUser(u); loadSchemas(); loadApprovalsCount(); showToast(`Welcome back, ${u.username}!`); }} />;
      }

      return (
        <div className="flex w-full min-h-screen">
          {/* SIDEBAR */}
          <Sidebar 
            siteName={SITE_NAME}
            user={user}
            schemas={schemas}
            currentTab={currentTab}
            currentModel={currentModel}
            pendingCount={pendingCount}
            onNavigate={(tab, model = null) => {
              setCurrentTab(tab);
              setCurrentModel(model);
            }}
            onLogout={handleLogout}
          />

          {/* MAIN CONTENT AREA */}
          <main className="flex-1 ml-64 p-8 min-h-screen bg-gradient-to-b from-oxide-950 via-slate-900 to-oxide-950">
            <TopNavbar currentTab={currentTab} currentModel={currentModel} schemas={schemas} />

            <div className="mt-8">
              {currentTab === 'dashboard' && (
                <DashboardView 
                  schemas={schemas} 
                  onNavigateModel={(m) => { setCurrentTab('model'); setCurrentModel(m); }} 
                />
              )}
              {currentTab === 'model' && currentModel && (
                <ModelTableView 
                  key={currentModel}
                  modelName={currentModel} 
                  schema={schemas.find(s => s.name.toLowerCase() === currentModel.toLowerCase())}
                  showToast={showToast}
                />
              )}
              {currentTab === 'ai-tuner' && (
                <AiTunerView showToast={showToast} />
              )}
              {currentTab === 'audit-logs' && (
                <AuditLogsView showToast={showToast} />
              )}
              {currentTab === 'approvals' && (
                <ApprovalQueueView 
                  showToast={showToast} 
                  onApprovalsUpdated={loadApprovalsCount} 
                />
              )}
              {currentTab === 'react-sdk' && (
                <ReactSdkView />
              )}
            </div>
          </main>

          {/* TOAST NOTIFICATION */}
          {toast && (
            <div className="fixed bottom-6 right-6 z-50 transition-all duration-300">
              <div className={`glass px-5 py-3.5 rounded-xl border shadow-2xl flex items-center space-x-3 text-sm font-medium ${
                toast.isError ? 'border-rose-500/50 text-rose-300' : 'border-emerald-500/50 text-emerald-300'
              }`}>
                <span>{toast.isError ? '⚠️' : '✨'}</span>
                <span>{toast.msg}</span>
              </div>
            </div>
          )}
        </div>
      );
    }

    // AUTH LOGIN MODAL
    function AuthModal({ onLoginSuccess }) {
      const [username, setUsername] = useState('admin');
      const [password, setPassword] = useState('admin');
      const [error, setError] = useState('');
      const [loading, setLoading] = useState(false);

      const handleSubmit = async (e) => {
        e.preventDefault();
        setError('');
        setLoading(true);
        try {
          const res = await apiRequest('/api/auth/login', {
            method: 'POST',
            body: JSON.stringify({ username, password })
          });
          if (res.success) {
            onLoginSuccess(res.session);
          } else {
            setError(res.message || 'Invalid login');
          }
        } catch (err) {
          setError('Server connection error');
        } finally {
          setLoading(false);
        }
      };

      return (
        <div className="fixed inset-0 z-50 bg-black/85 backdrop-blur-md flex items-center justify-center p-4">
          <div className="glass max-w-md w-full p-8 rounded-2xl shadow-2xl border border-slate-800 relative">
            <div className="flex items-center space-x-3 mb-6">
              <div className="w-10 h-10 rounded-xl bg-gradient-to-tr from-brand-500 via-react-400 to-ai-500 flex items-center justify-center font-bold text-black text-xl shadow-lg shadow-brand-500/20">
                ⚡
              </div>
              <div>
                <h2 className="text-xl font-bold tracking-tight text-white flex items-center space-x-2">
                  <span>Oxide_CG</span>
                  <span className="text-[10px] bg-ai-500/20 text-ai-400 border border-ai-500/30 px-2 py-0.5 rounded-full font-mono">AI Tuner</span>
                </h2>
                <p className="text-xs text-slate-400">Enterprise Multi-DB & React Engine in Rust</p>
              </div>
            </div>

            <form onSubmit={handleSubmit} className="space-y-4">
              <div>
                <label className="block text-xs font-semibold uppercase tracking-wider text-slate-300 mb-1.5">Username</label>
                <input 
                  type="text" 
                  value={username} 
                  onChange={(e) => setUsername(e.target.value)} 
                  className="w-full px-4 py-2.5 rounded-xl bg-slate-900/90 border border-slate-700 text-slate-100 focus:outline-none focus:border-brand-500 transition" 
                  required 
                />
              </div>
              <div>
                <label className="block text-xs font-semibold uppercase tracking-wider text-slate-300 mb-1.5">Password</label>
                <input 
                  type="password" 
                  value={password} 
                  onChange={(e) => setPassword(e.target.value)} 
                  className="w-full px-4 py-2.5 rounded-xl bg-slate-900/90 border border-slate-700 text-slate-100 focus:outline-none focus:border-brand-500 transition" 
                  required 
                />
              </div>

              {error && <div className="text-rose-400 text-sm font-medium">{error}</div>}

              <button 
                type="submit" 
                disabled={loading}
                className="w-full py-3 bg-gradient-to-r from-brand-500 to-emerald-400 hover:from-brand-600 hover:to-emerald-500 text-black font-bold rounded-xl transition shadow-lg shadow-brand-500/25 flex items-center justify-center space-x-2"
              >
                <span>{loading ? 'Authenticating...' : 'Sign In to Oxide_CG'}</span>
                <span>&rarr;</span>
              </button>

              <div className="text-center text-xs text-slate-500 pt-2 border-t border-slate-800">
                Default Credentials: <span className="text-slate-300 font-mono">admin / admin</span>
              </div>
            </form>
          </div>
        </div>
      );
    }

    // SIDEBAR COMPONENT
    function Sidebar({ siteName, user, schemas, currentTab, currentModel, pendingCount, onNavigate, onLogout }) {
      const [showProfileModal, setShowProfileModal] = useState(false);

      const groupedSchemas = useMemo(() => {
        const groups = {};
        schemas.forEach(s => {
          const cat = s.category || 'General';
          if (!groups[cat]) groups[cat] = [];
          groups[cat].push(s);
        });
        return groups;
      }, [schemas]);

      return (
        <aside className="w-64 glass border-r border-slate-800/80 flex flex-col fixed inset-y-0 z-40">
          <div className="p-5 border-b border-slate-800 flex items-center justify-between">
            <div className="flex items-center space-x-3">
              <div className="w-8 h-8 rounded-lg bg-gradient-to-tr from-brand-500 via-react-400 to-ai-500 flex items-center justify-center font-bold text-black text-lg shadow-md shadow-brand-500/20">⚡</div>
              <div>
                <div className="font-bold text-base tracking-tight text-white">{siteName}</div>
                <div className="text-[10px] text-ai-400 font-mono flex items-center space-x-1">
                  <span>🧠</span>
                  <span>AI Tuner & Multi-DB</span>
                </div>
              </div>
            </div>
          </div>

          <div className="flex-1 overflow-y-auto custom-scrollbar p-3 space-y-6">
            {/* CORE NAVIGATION */}
            <div>
              <div className="text-[11px] uppercase tracking-wider font-bold text-slate-500 px-3 mb-2">Core System</div>
              <nav className="space-y-1">
                <button 
                  onClick={() => onNavigate('dashboard')}
                  className={`w-full flex items-center space-x-3 px-3 py-2 rounded-lg text-sm font-medium transition ${
                    currentTab === 'dashboard' ? 'bg-slate-800 text-white border border-slate-700 font-semibold' : 'text-slate-300 hover:bg-slate-800/60 hover:text-white'
                  }`}
                >
                  <svg className="w-4 h-4 text-brand-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"/></svg>
                  <span>Dashboard</span>
                </button>
                <button 
                  onClick={() => onNavigate('ai-tuner')}
                  className={`w-full flex items-center space-x-3 px-3 py-2 rounded-lg text-sm font-medium transition ${
                    currentTab === 'ai-tuner' ? 'bg-slate-800 text-white border border-slate-700 font-semibold' : 'text-slate-300 hover:bg-slate-800/60 hover:text-white'
                  }`}
                >
                  <svg className="w-4 h-4 text-ai-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M13 10V3L4 14h7v7l9-11h-7z"/></svg>
                  <span>AI Tuner Hub</span>
                  <span className="ml-auto text-[9px] bg-ai-500/20 text-ai-300 border border-ai-500/30 px-1.5 py-0.5 rounded font-mono">AI</span>
                </button>
                <button 
                  onClick={() => onNavigate('audit-logs')}
                  className={`w-full flex items-center space-x-3 px-3 py-2 rounded-lg text-sm font-medium transition ${
                    currentTab === 'audit-logs' ? 'bg-slate-800 text-white border border-slate-700 font-semibold' : 'text-slate-300 hover:bg-slate-800/60 hover:text-white'
                  }`}
                >
                  <svg className="w-4 h-4 text-indigo-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"/></svg>
                  <span>Audit & Rollback</span>
                </button>
                <button 
                  onClick={() => onNavigate('approvals')}
                  className={`w-full flex items-center space-x-3 px-3 py-2 rounded-lg text-sm font-medium transition ${
                    currentTab === 'approvals' ? 'bg-slate-800 text-white border border-slate-700 font-semibold' : 'text-slate-300 hover:bg-slate-800/60 hover:text-white'
                  }`}
                >
                  <svg className="w-4 h-4 text-amber-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/></svg>
                  <span>Approval Queue</span>
                  {pendingCount > 0 && (
                    <span className="ml-auto text-xs bg-amber-500/20 text-amber-300 px-2 py-0.5 rounded-full font-bold">{pendingCount}</span>
                  )}
                </button>
              </nav>
            </div>

            {/* DYNAMIC REGISTERED MODELS */}
            {Object.entries(groupedSchemas).map(([category, models]) => (
              <div key={category}>
                <div className="text-[11px] uppercase tracking-wider font-bold text-slate-500 px-3 mb-2">{category}</div>
                <nav className="space-y-1">
                  {models.map(m => {
                    const isSelected = currentTab === 'model' && currentModel === m.name.toLowerCase();
                    return (
                      <button
                        key={m.name}
                        onClick={() => onNavigate('model', m.name.toLowerCase())}
                        className={`w-full flex items-center space-x-3 px-3 py-2 rounded-lg text-sm font-medium transition ${
                          isSelected ? 'bg-slate-800 text-white border border-slate-700 font-semibold' : 'text-slate-300 hover:bg-slate-800/60 hover:text-white'
                        }`}
                      >
                        <svg className="w-4 h-4 text-slate-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"/></svg>
                        <span className="truncate">{m.display_name}</span>
                      </button>
                    );
                  })}
                </nav>
              </div>
            ))}

            {/* DEVELOPER SECTION */}
            <div>
              <div className="text-[11px] uppercase tracking-wider font-bold text-slate-500 px-3 mb-2">React & API Tools</div>
              <nav className="space-y-1">
                <button
                  onClick={() => onNavigate('react-sdk')}
                  className={`w-full flex items-center space-x-3 px-3 py-2 rounded-lg text-sm font-medium transition ${
                    currentTab === 'react-sdk' ? 'bg-slate-800 text-white border border-slate-700 font-semibold' : 'text-slate-300 hover:bg-slate-800/60 hover:text-white'
                  }`}
                >
                  <svg className="w-4 h-4 text-react-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4"/></svg>
                  <span>React & Frontend SDKs</span>
                </button>
                <a 
                  href="/swagger" 
                  target="_blank" 
                  className="w-full flex items-center space-x-3 px-3 py-2 rounded-lg text-sm font-medium transition text-slate-300 hover:bg-slate-800/60 hover:text-white"
                >
                  <svg className="w-4 h-4 text-emerald-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/></svg>
                  <span>Swagger OpenAPI</span>
                  <span className="ml-auto text-[10px] bg-slate-800 text-slate-400 px-1.5 py-0.5 rounded">v3.1</span>
                </a>
              </nav>
            </div>
          </div>

          {/* USER FOOTER */}
          <div className="p-4 border-t border-slate-800 flex items-center justify-between">
            <button 
              onClick={() => setShowProfileModal(true)}
              className="flex items-center space-x-3 overflow-hidden text-left hover:bg-slate-800/60 p-1.5 rounded-xl transition flex-1 mr-2"
              title="Click to view Account & Security Details"
            >
              <div className="w-8 h-8 rounded-full bg-gradient-to-tr from-brand-500 to-emerald-400 flex items-center justify-center font-bold text-xs text-black shadow-sm">
                {user.username.charAt(0).toUpperCase()}
              </div>
              <div className="truncate">
                <div className="text-xs font-semibold truncate text-white flex items-center space-x-1">
                  <span>{user.username}</span>
                  <span className="text-[9px] text-slate-500">&bull; Profile</span>
                </div>
                <div className="text-[10px] text-brand-400 font-mono">{user.role}</div>
              </div>
            </button>
            <button onClick={onLogout} title="Logout" className="text-slate-400 hover:text-rose-400 p-1.5 rounded-lg hover:bg-slate-800 transition">
              <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1"/></svg>
            </button>
          </div>

          {/* USER PROFILE MODAL */}
          {showProfileModal && (
            <div 
              onClick={(e) => { if (e.target === e.currentTarget) setShowProfileModal(false); }}
              className="fixed inset-0 z-50 bg-black/80 backdrop-blur-sm flex items-center justify-center p-4"
            >
              <div className="glass max-w-md w-full p-6 rounded-2xl border border-slate-800 space-y-4">
                <div className="flex items-center justify-between border-b border-slate-800 pb-3">
                  <h3 className="text-base font-bold text-white flex items-center space-x-2">
                    <span>🛡️ Account Profile & RBAC Info</span>
                  </h3>
                  <button onClick={() => setShowProfileModal(false)} className="text-slate-400 hover:text-white p-1">✕</button>
                </div>
                <div className="space-y-3 text-xs">
                  <div className="flex justify-between p-2.5 rounded-lg bg-slate-900/60 border border-slate-800">
                    <span className="text-slate-400">Username:</span>
                    <span className="text-white font-mono font-bold">{user.username}</span>
                  </div>
                  <div className="flex justify-between p-2.5 rounded-lg bg-slate-900/60 border border-slate-800">
                    <span className="text-slate-400">Assigned Role:</span>
                    <span className="text-emerald-400 font-mono font-bold">{user.role}</span>
                  </div>
                  <div className="flex justify-between p-2.5 rounded-lg bg-slate-900/60 border border-slate-800">
                    <span className="text-slate-400">Security Status:</span>
                    <span className="text-brand-400 font-bold">Active & Authenticated</span>
                  </div>
                  <div className="p-3 rounded-lg bg-slate-900/80 border border-slate-800 space-y-1">
                    <span className="text-[11px] font-semibold text-slate-300 block">RBAC Capabilities:</span>
                    <div className="flex flex-wrap gap-1.5 pt-1">
                      <span className="px-2 py-0.5 rounded bg-emerald-500/20 text-emerald-300 text-[10px] font-mono">READ: ALLOWED</span>
                      <span className="px-2 py-0.5 rounded bg-emerald-500/20 text-emerald-300 text-[10px] font-mono">CREATE: ALLOWED</span>
                      <span className="px-2 py-0.5 rounded bg-emerald-500/20 text-emerald-300 text-[10px] font-mono">UPDATE: ALLOWED</span>
                      <span className="px-2 py-0.5 rounded bg-emerald-500/20 text-emerald-300 text-[10px] font-mono">DELETE: ALLOWED</span>
                      <span className="px-2 py-0.5 rounded bg-amber-500/20 text-amber-300 text-[10px] font-mono">APPROVAL: MANAGER+</span>
                    </div>
                  </div>
                </div>
                <div className="pt-2 flex justify-end">
                  <button onClick={() => setShowProfileModal(false)} className="px-4 py-2 bg-slate-800 hover:bg-slate-700 text-white rounded-xl text-xs font-semibold">Close</button>
                </div>
              </div>
            </div>
          )}
        </aside>
      );
    }

    // TOP NAVBAR
    function TopNavbar({ currentTab, currentModel, schemas }) {
      const activeSchema = schemas.find(s => s.name.toLowerCase() === (currentModel || '').toLowerCase());
      
      let title = 'Dashboard Overview';
      let subtitle = 'Oxide_CG Fast React Admin Engine';
      if (currentTab === 'model' && activeSchema) {
        title = activeSchema.display_name;
        subtitle = `Manage and query ${activeSchema.table_name}`;
      } else if (currentTab === 'ai-tuner') {
        title = 'AI Tuner & Database Intelligence';
        subtitle = 'Real-time query telemetry, index recommendations, and automated risk scoring';
      } else if (currentTab === 'audit-logs') {
        title = 'Audit Trail & Time-Travel';
        subtitle = 'Complete change history with 1-click snapshot rollback';
      } else if (currentTab === 'approvals') {
        title = 'Approval Workflow Queue';
        subtitle = 'Review sensitive field modifications before they go live';
      } else if (currentTab === 'react-sdk') {
        title = 'React Ecosystem & TypeScript SDK';
        subtitle = 'Auto-generated React client, query hooks, and Next.js integration';
      }

      return (
        <header className="flex items-center justify-between pb-6 border-b border-slate-800">
          <div>
            <h1 className="text-2xl font-bold tracking-tight text-white">{title}</h1>
            <p className="text-xs text-slate-400 mt-1">{subtitle}</p>
          </div>
          <div className="flex items-center space-x-3">
            <span className="inline-flex items-center px-3 py-1 rounded-full text-xs font-medium bg-ai-500/10 text-ai-300 border border-ai-500/20">
              <span className="w-1.5 h-1.5 rounded-full bg-ai-400 mr-2 animate-pulse"></span>
              AI Tuner Active &bull; Multi-DB Core
            </span>
          </div>
        </header>
      );
    }

    // DASHBOARD VIEW
    function DashboardView({ schemas, onNavigateModel }) {
      return (
        <div className="space-y-8">
          <div className="grid grid-cols-1 md:grid-cols-4 gap-6">
            <div className="glass p-6 rounded-2xl border border-slate-800">
              <div className="text-xs font-semibold uppercase tracking-wider text-slate-400">Registered Models</div>
              <div className="text-3xl font-bold mt-2 text-white">{schemas.length}</div>
              <div className="text-xs text-brand-400 mt-2 font-mono">Zero runtime reflection</div>
            </div>
            <div className="glass p-6 rounded-2xl border border-slate-800">
              <div className="text-xs font-semibold uppercase tracking-wider text-slate-400">Response Latency</div>
              <div className="text-3xl font-bold mt-2 text-emerald-400">&lt; 0.5 ms</div>
              <div className="text-xs text-slate-400 mt-2">Axum 0.7 + Tokio + Multi-DB</div>
            </div>
            <div className="glass p-6 rounded-2xl border border-slate-800">
              <div className="text-xs font-semibold uppercase tracking-wider text-slate-400">AI Tuner Engine</div>
              <div className="text-3xl font-bold mt-2 text-ai-400">Online</div>
              <div className="text-xs text-slate-400 mt-2">Index Advisor & Risk Scorer</div>
            </div>
            <div className="glass p-6 rounded-2xl border border-slate-800">
              <div className="text-xs font-semibold uppercase tracking-wider text-slate-400">React Frontend</div>
              <div className="text-3xl font-bold mt-2 text-react-400">React 18</div>
              <div className="text-xs text-slate-400 mt-2">Embedded Single-Page App</div>
            </div>
          </div>

          <div className="glass p-6 rounded-2xl border border-slate-800">
            <h3 className="text-lg font-bold mb-4 text-white">Registered Models & Data Tables</h3>
            <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
              {schemas.map(s => (
                <button
                  key={s.name}
                  onClick={() => onNavigateModel(s.name.toLowerCase())}
                  className="p-5 rounded-xl bg-slate-900/90 border border-slate-800 hover:border-brand-500/50 text-left transition flex items-center justify-between group shadow-sm hover:shadow-lg hover:shadow-brand-500/5"
                >
                  <div>
                    <div className="font-bold text-white group-hover:text-brand-400 transition text-base">{s.display_name}</div>
                    <div className="text-xs text-slate-500 font-mono mt-1">{s.table_name} &bull; {s.fields.length} fields</div>
                  </div>
                  <span className="text-slate-600 group-hover:text-brand-400 font-bold text-lg">&rarr;</span>
                </button>
              ))}
            </div>
          </div>
        </div>
      );
    }

    // AI TUNER & DATABASE INTELLIGENCE VIEW
    function AiTunerView({ showToast }) {
      const [report, setReport] = useState(null);
      const [loading, setLoading] = useState(true);
      const [applyingIndex, setApplyingIndex] = useState(null);

      const fetchReport = useCallback(async () => {
        setLoading(true);
        try {
          const res = await apiRequest('/api/ai/report');
          if (res.success) {
            setReport(res.report);
          }
        } catch (e) {
          showToast('Failed to load AI Tuner telemetry', true);
        } finally {
          setLoading(false);
        }
      }, [showToast]);

      useEffect(() => {
        fetchReport();
      }, [fetchReport]);

      const handleApplyIndex = async (rec) => {
        setApplyingIndex(rec.id);
        try {
          const res = await apiRequest(`/api/ai/indexes/apply?table=${rec.table_name}&column=${rec.column}`, { method: 'POST' });
          if (res.success) {
            showToast(`Index created on ${rec.table_name}(${rec.column})!`);
            fetchReport();
          } else {
            showToast(res.message || 'Failed to apply index', true);
          }
        } catch (e) {
          showToast('Error applying index', true);
        } finally {
          setApplyingIndex(null);
        }
      };

      if (loading || !report) {
        return <div className="glass p-8 rounded-2xl text-center text-slate-400 font-medium">Analyzing database workloads & telemetry...</div>;
      }

      return (
        <div className="space-y-8">
          {/* LATENCY METRICS */}
          <div className="grid grid-cols-1 md:grid-cols-4 gap-6">
            <div className="glass p-6 rounded-2xl border border-slate-800">
              <div className="text-xs font-semibold uppercase tracking-wider text-slate-400">Total Analyzed Queries</div>
              <div className="text-3xl font-bold mt-2 text-white">{report.total_queries_analyzed}</div>
              <div className="text-xs text-brand-400 mt-2 font-mono">{report.qps} QPS Average</div>
            </div>
            <div className="glass p-6 rounded-2xl border border-slate-800">
              <div className="text-xs font-semibold uppercase tracking-wider text-slate-400">p50 Median Latency</div>
              <div className="text-3xl font-bold mt-2 text-emerald-400">{report.p50_latency_ms} ms</div>
              <div className="text-xs text-slate-400 mt-2">50% of queries under this time</div>
            </div>
            <div className="glass p-6 rounded-2xl border border-slate-800">
              <div className="text-xs font-semibold uppercase tracking-wider text-slate-400">p95 Latency</div>
              <div className="text-3xl font-bold mt-2 text-amber-400">{report.p95_latency_ms} ms</div>
              <div className="text-xs text-slate-400 mt-2">95% of queries under this time</div>
            </div>
            <div className="glass p-6 rounded-2xl border border-slate-800">
              <div className="text-xs font-semibold uppercase tracking-wider text-slate-400">p99 Tail Latency</div>
              <div className="text-3xl font-bold mt-2 text-ai-400">{report.p99_latency_ms} ms</div>
              <div className="text-xs text-slate-400 mt-2">Tail latency outlier threshold</div>
            </div>
          </div>

          {/* AI WORKLOAD SUMMARY */}
          <div className="glass p-6 rounded-2xl border border-slate-800 bg-gradient-to-r from-slate-900 via-slate-900/90 to-ai-950/20">
            <div className="flex items-center space-x-3 mb-2">
              <span className="text-xl">🧠</span>
              <h3 className="text-base font-bold text-white">AI Workload Assessment</h3>
            </div>
            <p className="text-sm text-slate-300">{report.workload_summary}</p>
          </div>

          {/* AI INDEX ADVISOR */}
          <div className="glass p-6 rounded-2xl border border-slate-800 space-y-4">
            <div className="flex items-center justify-between">
              <div>
                <h3 className="text-lg font-bold text-white flex items-center space-x-2">
                  <span>⚡ AI Index Advisor & Recommendations</span>
                  <span className="text-xs bg-ai-500/20 text-ai-300 px-2 py-0.5 rounded-full font-mono font-bold">
                    {report.recommendations.length} Active
                  </span>
                </h3>
                <p className="text-xs text-slate-400 mt-0.5">The AI Tuner predicts index optimizations to eliminate full-table scans</p>
              </div>
              <button onClick={fetchReport} className="px-3 py-1.5 bg-slate-800 hover:bg-slate-700 rounded-lg text-xs font-medium text-slate-300 transition">
                Refresh Telemetry ↺
              </button>
            </div>

            <div className="space-y-3">
              {report.recommendations.length === 0 ? (
                <div className="p-8 text-center text-slate-500 font-sans border border-slate-800 rounded-xl">
                  No unindexed high-frequency queries detected. Database indexes are fully optimized!
                </div>
              ) : (
                report.recommendations.map(rec => (
                  <div key={rec.id} className="p-4 rounded-xl bg-slate-900/90 border border-slate-800 flex flex-col md:flex-row md:items-center justify-between gap-4">
                    <div className="space-y-1 flex-1">
                      <div className="flex items-center space-x-2">
                        <span className="font-bold text-white text-sm">{rec.table_name}.{rec.column}</span>
                        <span className="text-[10px] bg-emerald-500/20 text-emerald-300 font-mono font-semibold px-2 py-0.5 rounded">
                          {rec.estimated_speedup}
                        </span>
                      </div>
                      <p className="text-xs text-slate-400">{rec.reason}</p>
                      <code className="text-[11px] text-slate-500 font-mono block pt-1">{rec.ddl}</code>
                    </div>

                    <div>
                      {rec.is_applied ? (
                        <span className="px-3 py-1.5 bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 rounded-xl text-xs font-semibold">
                          ✅ Index Active
                        </span>
                      ) : (
                        <button
                          disabled={applyingIndex === rec.id}
                          onClick={() => handleApplyIndex(rec)}
                          className="px-4 py-2 bg-gradient-to-r from-ai-500 to-indigo-500 hover:from-ai-600 hover:to-indigo-600 text-white font-bold text-xs rounded-xl transition shadow-lg shadow-ai-500/20"
                        >
                          {applyingIndex === rec.id ? 'Applying...' : 'Auto-Apply Index ✨'}
                        </button>
                      )}
                    </div>
                  </div>
                ))
              )}
            </div>
          </div>
        </div>
      );
    }

    // DYNAMIC MODEL DATA TABLE VIEW
    function ModelTableView({ modelName, schema, showToast }) {
      const [records, setRecords] = useState([]);
      const [total, setTotal] = useState(0);
      const [limit, setLimit] = useState(25);
      const [offset, setOffset] = useState(0);
      const [searchQuery, setSearchQuery] = useState('');
      const [loading, setLoading] = useState(true);
      const [editingRecord, setEditingRecord] = useState(null);
      const [isModalOpen, setIsModalOpen] = useState(false);

      const fetchRecords = useCallback(async () => {
        if (!schema) return;
        setLoading(true);
        let url = `/api/d/${modelName}?$limit=${limit}&$offset=${offset}`;
        if (searchQuery) {
          url += `&$search=${encodeURIComponent(searchQuery)}`;
        }
        try {
          const res = await apiRequest(url);
          if (res.success) {
            setRecords(res.data);
            setTotal(res.total);
          }
        } catch (e) {
          showToast('Failed to load records', true);
        } finally {
          setLoading(false);
        }
      }, [modelName, schema, limit, offset, searchQuery, showToast]);

      useEffect(() => {
        fetchRecords();
      }, [fetchRecords]);

      const handleDelete = async (id) => {
        if (!confirm(`Are you sure you want to delete record #${id}?`)) return;
        try {
          const res = await apiRequest(`/api/d/${modelName}/${id}`, { method: 'DELETE' });
          if (res.success) {
            showToast(`Record #${id} deleted (Snapshot saved to audit log)`);
            fetchRecords();
          } else {
            showToast(res.message || 'Failed to delete record', true);
          }
        } catch (e) {
          showToast('Delete error', true);
        }
      };

      if (!schema) return null;
      const visibleFields = schema.fields.filter(f => f.list_display);

      return (
        <div className="glass p-6 rounded-2xl border border-slate-800 space-y-6">
          {/* ACTION BAR */}
          <div className="flex flex-col md:flex-row md:items-center justify-between gap-4">
            <div className="relative flex-1 max-w-md">
              <input
                type="text"
                placeholder={`Search ${schema.display_name}...`}
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                className="w-full pl-10 pr-4 py-2.5 rounded-xl bg-slate-900/90 border border-slate-700 text-sm focus:outline-none focus:border-brand-500 text-slate-100"
              />
              <svg className="w-4 h-4 text-slate-400 absolute left-3.5 top-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/></svg>
            </div>

            <button
              onClick={() => { setEditingRecord(null); setIsModalOpen(true); }}
              className="px-4 py-2.5 bg-gradient-to-r from-brand-500 to-emerald-400 hover:from-brand-600 hover:to-emerald-500 text-black font-bold text-sm rounded-xl transition shadow-lg shadow-brand-500/20 flex items-center space-x-2"
            >
              <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M12 4v16m8-8H4"/></svg>
              <span>New {schema.name}</span>
            </button>
          </div>

          {/* TABLE */}
          <div className="overflow-x-auto custom-scrollbar border border-slate-800 rounded-xl">
            <table className="w-full text-left text-sm">
              <thead className="bg-slate-900/90 text-xs uppercase tracking-wider text-slate-400 border-b border-slate-800">
                <tr>
                  {visibleFields.map(f => (
                    <th key={f.name} className="px-4 py-3 font-semibold">{f.display_name}</th>
                  ))}
                  <th className="px-4 py-3 font-semibold text-right">Actions</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-slate-800/60 font-mono text-xs">
                {loading ? (
                  <tr><td colSpan="100" className="p-8 text-center text-slate-500 font-sans">Loading records...</td></tr>
                ) : records.length === 0 ? (
                  <tr><td colSpan="100" className="p-8 text-center text-slate-500 font-sans">No records found. Click "+ New" to add one.</td></tr>
                ) : (
                  records.map(r => (
                    <tr key={r.id} className="hover:bg-slate-800/40 transition">
                      {visibleFields.map(f => {
                        const val = r[f.name];
                        let content = val === null || val === undefined ? <span className="text-slate-600 font-sans">null</span> : String(val);

                        if (typeof val === 'boolean') {
                          content = val ? (
                            <span className="px-2 py-0.5 rounded-full text-[10px] bg-emerald-500/20 text-emerald-300 font-sans font-semibold">TRUE</span>
                          ) : (
                            <span className="px-2 py-0.5 rounded-full text-[10px] bg-slate-700 text-slate-400 font-sans font-semibold">FALSE</span>
                          );
                        } else if (f.field_type.kind === 'Money') {
                          content = <span className="text-emerald-400 font-semibold">${Number(val || 0).toFixed(2)}</span>;
                        } else if (f.field_type.kind === 'ProgressBar') {
                          const maxVal = f.field_type.config?.max || 100;
                          const pct = Math.min(100, Math.max(0, (Number(val || 0) / maxVal) * 100));
                          content = (
                            <div className="flex items-center space-x-2">
                              <div className="w-20 bg-slate-800 rounded-full h-2 overflow-hidden">
                                <div className="bg-brand-500 h-full" style={{ width: `${pct}%` }}></div>
                              </div>
                              <span>{val}</span>
                            </div>
                          );
                        }

                        return <td key={f.name} className="px-4 py-3 text-slate-300 truncate max-w-xs">{content}</td>;
                      })}
                      <td className="px-4 py-3 text-right space-x-3 font-sans">
                        <button onClick={() => { setEditingRecord(r); setIsModalOpen(true); }} className="text-brand-400 hover:text-brand-300 font-semibold text-xs">Edit</button>
                        <button onClick={() => handleDelete(r.id)} className="text-rose-400 hover:text-rose-300 font-semibold text-xs">Delete</button>
                      </td>
                    </tr>
                  ))
                )}
              </tbody>
            </table>
          </div>

          {/* PAGINATION */}
          <div className="flex items-center justify-between text-xs text-slate-400 pt-2">
            <div>
              Showing {total === 0 ? 0 : offset + 1} - {Math.min(offset + limit, total)} of {total} records
            </div>
            <div className="flex space-x-2">
              <button
                disabled={offset === 0}
                onClick={() => setOffset(Math.max(0, offset - limit))}
                className="px-3 py-1.5 bg-slate-800 hover:bg-slate-700 disabled:opacity-30 rounded-lg transition font-medium text-white"
              >
                Previous
              </button>
              <button
                disabled={offset + limit >= total}
                onClick={() => setOffset(offset + limit)}
                className="px-3 py-1.5 bg-slate-800 hover:bg-slate-700 disabled:opacity-30 rounded-lg transition font-medium text-white"
              >
                Next
              </button>
            </div>
          </div>

          {/* DYNAMIC REACT FORM MODAL */}
          {isModalOpen && (
            <RecordModal
              schema={schema}
              record={editingRecord}
              onClose={() => setIsModalOpen(false)}
              onSaved={() => { setIsModalOpen(false); fetchRecords(); showToast(editingRecord ? 'Record updated!' : 'Record created!'); }}
              showToast={showToast}
            />
          )}
        </div>
      );
    }

    // DYNAMIC REACT RECORD MODAL FORM (ENHANCED COMPLETE WIDGETS)
    function RecordModal({ schema, record, onClose, onSaved, showToast }) {
      const [formData, setFormData] = useState(() => {
        const initial = {};
        schema.fields.forEach(f => {
          if (f.name === 'id' || f.name === 'created_at' || f.name === 'updated_at') return;
          if (record && record[f.name] !== undefined && record[f.name] !== null) {
            initial[f.name] = record[f.name];
          } else if (f.default_value !== undefined && f.default_value !== null) {
            initial[f.name] = f.default_value;
          } else if (f.field_type.kind === 'Boolean') {
            initial[f.name] = false;
          } else if (f.field_type.kind === 'ProgressBar') {
            initial[f.name] = 0;
          } else if (f.field_type.kind === 'Enum') {
            initial[f.name] = (f.field_type.config?.choices || [])[0] || 'Medium';
          } else if (f.name === 'category') {
            initial[f.name] = 'General';
          } else if (f.name === 'title' || f.name === 'name') {
            initial[f.name] = `New ${schema.name} Task`;
          } else if (f.name === 'sku') {
            initial[f.name] = `SKU-${Math.floor(1000 + Math.random() * 9000)}`;
          } else if (f.field_type.kind === 'Money' || f.name === 'price') {
            initial[f.name] = 29.99;
          } else if (f.field_type.kind === 'Email') {
            initial[f.name] = 'user@example.com';
          } else {
            initial[f.name] = '';
          }
        });
        return initial;
      });
      const [saving, setSaving] = useState(false);
      const [errorMessage, setErrorMessage] = useState('');

      // Keyboard Escape key to close modal
      useEffect(() => {
        const handleKeyDown = (e) => {
          if (e.key === 'Escape') onClose();
        };
        window.addEventListener('keydown', handleKeyDown);
        return () => window.removeEventListener('keydown', handleKeyDown);
      }, [onClose]);

      const handleChange = (field, value) => {
        setErrorMessage('');
        setFormData(prev => ({ ...prev, [field]: value }));
      };

      const handleSubmit = async (e) => {
        e.preventDefault();
        setSaving(true);
        setErrorMessage('');

        const payload = {};
        for (const f of schema.fields) {
          if (f.name === 'id' || f.name === 'created_at' || f.name === 'updated_at') continue;
          const val = formData[f.name];

          // Validate required fields
          if (f.required && (val === undefined || val === null || val === '')) {
            setErrorMessage(`Field "${f.display_name}" is required.`);
            setSaving(false);
            return;
          }

          if (f.field_type.kind === 'Integer' || f.field_type.kind === 'ForeignKey') {
            payload[f.name] = val !== '' ? (parseInt(val, 10) || 0) : null;
          } else if (f.field_type.kind === 'Float' || f.field_type.kind === 'Money' || f.field_type.kind === 'ProgressBar') {
            payload[f.name] = val !== '' ? (parseFloat(val) || 0.0) : null;
          } else if (f.field_type.kind === 'Boolean') {
            payload[f.name] = Boolean(val);
          } else {
            payload[f.name] = val;
          }
        }

        const isEdit = record !== null && record !== undefined;
        const url = isEdit ? `/api/d/${schema.name.toLowerCase()}/${record.id}` : `/api/d/${schema.name.toLowerCase()}`;
        const method = isEdit ? 'PUT' : 'POST';

        try {
          const res = await apiRequest(url, { method, body: JSON.stringify(payload) });
          if (res.success) {
            onSaved();
          } else {
            const errText = res.error?.message || res.message || 'Error saving record';
            setErrorMessage(errText);
            showToast(errText, true);
          }
        } catch (err) {
          const errText = err.message || 'Failed to save record';
          setErrorMessage(errText);
          showToast(errText, true);
        } finally {
          setSaving(false);
        }
      };

      return (
        <div 
          onClick={(e) => { if (e.target === e.currentTarget) onClose(); }}
          className="fixed inset-0 z-50 bg-black/80 backdrop-blur-sm flex items-center justify-center p-4 animate-in fade-in duration-150"
        >
          <div className="glass max-w-2xl w-full max-h-[90vh] flex flex-col rounded-2xl shadow-2xl border border-slate-800 animate-in zoom-in-95 duration-200">
            {/* MODAL HEADER */}
            <div className="p-6 border-b border-slate-800 flex items-center justify-between bg-slate-900/60 rounded-t-2xl">
              <div className="flex items-center space-x-3">
                <div className="w-9 h-9 rounded-xl bg-brand-500/20 text-brand-400 border border-brand-500/30 flex items-center justify-center font-bold text-base shadow-sm">
                  {record ? '✏️' : '➕'}
                </div>
                <div>
                  <h3 className="text-lg font-bold text-white flex items-center space-x-2">
                    <span>{record ? `Edit ${schema.name} #${record.id}` : `Create New ${schema.name}`}</span>
                  </h3>
                  <p className="text-xs text-slate-400">Database Table: <code className="text-brand-400 font-mono">{schema.table_name}</code> &bull; {schema.fields.length} schema fields</p>
                </div>
              </div>
              <div className="flex items-center space-x-2">
                {!record && (
                  <button
                    type="button"
                    onClick={() => {
                      const sample = {};
                      schema.fields.forEach(f => {
                        if (f.name === 'id' || f.name === 'created_at' || f.name === 'updated_at') return;
                        if (f.field_type.kind === 'Boolean') sample[f.name] = true;
                        else if (f.field_type.kind === 'ProgressBar') sample[f.name] = 75;
                        else if (f.field_type.kind === 'Money' || f.name === 'price') sample[f.name] = 99.99;
                        else if (f.field_type.kind === 'Enum') sample[f.name] = (f.field_type.config?.choices || [])[1] || 'High';
                        else if (f.name === 'category') sample[f.name] = 'Productivity';
                        else if (f.name === 'title' || f.name === 'name') sample[f.name] = 'Deploy Oxide_CG Microservice to Production';
                        else if (f.name === 'description') sample[f.name] = 'Automated high-performance task managed by Oxide_CG Rust backend.';
                        else sample[f.name] = 'Sample Data';
                      });
                      setFormData(sample);
                      setErrorMessage('');
                    }}
                    className="px-2.5 py-1 bg-brand-500/10 hover:bg-brand-500/20 text-brand-400 border border-brand-500/30 rounded-lg text-xs font-semibold transition"
                    title="Auto-fill sample data for rapid testing"
                  >
                    ✨ Auto-Fill Sample
                  </button>
                )}
                <button onClick={onClose} title="Close (Esc)" className="text-slate-400 hover:text-white p-2 rounded-lg hover:bg-slate-800 transition">
                  <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M6 18L18 6M6 6l12 12"/></svg>
                </button>
              </div>
            </div>

            {/* ERROR BANNER */}
            {errorMessage && (
              <div className="mx-6 mt-4 p-3.5 bg-rose-500/15 border border-rose-500/40 rounded-xl text-rose-300 text-xs font-medium flex items-center space-x-2 animate-in fade-in duration-200">
                <span className="text-sm">⚠️</span>
                <span className="flex-1">{errorMessage}</span>
              </div>
            )}

            {/* MODAL FORM BODY */}
            <form onSubmit={handleSubmit} className="flex-1 overflow-y-auto custom-scrollbar p-6 space-y-5">
              {schema.fields.filter(f => f.name !== 'id' && f.name !== 'created_at' && f.name !== 'updated_at').map((f, idx) => {
                const isBool = f.field_type.kind === 'Boolean';
                const isProgress = f.field_type.kind === 'ProgressBar';
                const isMoney = f.field_type.kind === 'Money';
                const isHtml = f.field_type.kind === 'Html' || f.field_type.kind === 'Markdown';
                const isEnum = f.field_type.kind === 'Enum';
                const isJson = f.field_type.kind === 'Json';
                const isImage = f.field_type.kind === 'Image' || f.field_type.kind === 'File';

                return (
                  <div key={f.name} className="space-y-1.5 p-3.5 rounded-xl bg-slate-900/40 border border-slate-800/80 hover:border-slate-700 transition">
                    <div className="flex items-center justify-between">
                      <label className="block text-xs font-semibold uppercase tracking-wider text-slate-300 flex items-center">
                        <span>{f.display_name}</span>
                        {f.required && <span className="text-rose-400 ml-1 font-bold">*</span>}
                        {f.requires_approval && (
                          <span className="ml-2 text-[10px] px-2 py-0.5 rounded bg-amber-500/20 text-amber-300 font-mono border border-amber-500/30">
                            🛡️ Approval Required
                          </span>
                        )}
                      </label>
                      <span className="text-[10px] text-slate-500 font-mono">{f.name}</span>
                    </div>

                    {/* WIDGET 1: BOOLEAN TOGGLE */}
                    {isBool ? (
                      <label className="flex items-center space-x-3 cursor-pointer py-1.5">
                        <input
                          type="checkbox"
                          checked={Boolean(formData[f.name])}
                          onChange={(e) => handleChange(f.name, e.target.checked)}
                          className="w-5 h-5 rounded text-brand-500 bg-slate-900 border-slate-700 cursor-pointer focus:ring-0"
                        />
                        <span className="text-sm font-medium text-slate-200">
                          {Boolean(formData[f.name]) ? 'Active / Enabled (TRUE)' : 'Disabled / Inactive (FALSE)'}
                        </span>
                      </label>
                    ) : isProgress ? (
                      /* WIDGET 2: PROGRESS BAR WITH LIVE SLIDER */
                      <div className="space-y-2 pt-1">
                        <div className="flex items-center justify-between text-xs font-mono">
                          <span className="text-slate-400">Progress: {formData[f.name] || 0} / {f.field_type.config?.max || 100}</span>
                          <span className="text-brand-400 font-bold bg-brand-500/10 border border-brand-500/20 px-2 py-0.5 rounded">
                            {Math.round(((Number(formData[f.name]) || 0) / (f.field_type.config?.max || 100)) * 100)}% Complete
                          </span>
                        </div>
                        <input
                          type="range"
                          min="0"
                          max={f.field_type.config?.max || 100}
                          value={formData[f.name] || 0}
                          onChange={(e) => handleChange(f.name, e.target.value)}
                          className="w-full accent-brand-500 cursor-pointer"
                        />
                      </div>
                    ) : isMoney ? (
                      /* WIDGET 3: MONEY WITH CURRENCY BADGE */
                      <div className="relative">
                        <span className="absolute left-3.5 top-2.5 text-xs font-bold text-emerald-400 font-mono">
                          {f.field_type.config?.currency || '$'}
                        </span>
                        <input
                          type="number"
                          step="0.01"
                          value={formData[f.name] !== undefined ? formData[f.name] : ''}
                          onChange={(e) => handleChange(f.name, e.target.value)}
                          placeholder="0.00"
                          className="w-full pl-8 pr-4 py-2.5 rounded-xl bg-slate-900 border border-slate-700 text-sm text-slate-100 focus:border-brand-500 font-mono"
                          required={f.required}
                        />
                      </div>
                    ) : isHtml ? (
                      /* WIDGET 4: HTML & MARKDOWN TEXTAREA */
                      <textarea
                        rows="4"
                        value={formData[f.name] || ''}
                        onChange={(e) => handleChange(f.name, e.target.value)}
                        placeholder="Enter HTML or Markdown formatted content..."
                        className="w-full px-4 py-2.5 rounded-xl bg-slate-900 border border-slate-700 text-sm text-slate-100 focus:border-brand-500 font-mono text-xs leading-relaxed"
                      />
                    ) : isEnum ? (
                      /* WIDGET 5: ENUM SELECT */
                      <select
                        value={formData[f.name] || ''}
                        onChange={(e) => handleChange(f.name, e.target.value)}
                        className="w-full px-4 py-2.5 rounded-xl bg-slate-900 border border-slate-700 text-sm text-slate-100 focus:border-brand-500 cursor-pointer"
                      >
                        {(f.field_type.config?.choices || []).map(choice => (
                          <option key={choice} value={choice}>{choice}</option>
                        ))}
                      </select>
                    ) : isJson ? (
                      /* WIDGET 6: JSON EDITOR */
                      <textarea
                        rows="3"
                        value={typeof formData[f.name] === 'object' ? JSON.stringify(formData[f.name], null, 2) : formData[f.name] || ''}
                        onChange={(e) => handleChange(f.name, e.target.value)}
                        placeholder="{}"
                        className="w-full px-4 py-2.5 rounded-xl bg-slate-900 border border-slate-700 text-sm text-slate-100 focus:border-brand-500 font-mono text-xs"
                      />
                    ) : isImage ? (
                      /* WIDGET 7: IMAGE / FILE PATH WITH PREVIEW */
                      <div className="space-y-2">
                        <input
                          type="text"
                          value={formData[f.name] || ''}
                          onChange={(e) => handleChange(f.name, e.target.value)}
                          placeholder="/uploads/images/sample.jpg"
                          className="w-full px-4 py-2.5 rounded-xl bg-slate-900 border border-slate-700 text-sm text-slate-100 focus:border-brand-500"
                        />
                        {formData[f.name] && formData[f.name].startsWith('http') && (
                          <div className="mt-2 p-2 bg-slate-950 rounded-xl border border-slate-800 inline-block">
                            <img src={formData[f.name]} alt="Preview" className="h-16 w-auto rounded object-cover" />
                          </div>
                        )}
                      </div>
                    ) : (
                      /* WIDGET 8: STANDARD TEXT / NUMBER INPUT */
                      <input
                        autoFocus={idx === 0}
                        type={f.field_type.kind === 'Password' ? 'password' : (f.field_type.kind === 'Integer' || f.field_type.kind === 'Float' ? 'number' : 'text')}
                        step={f.field_type.kind === 'Float' ? '0.01' : undefined}
                        value={formData[f.name] !== undefined ? formData[f.name] : ''}
                        onChange={(e) => handleChange(f.name, e.target.value)}
                        placeholder={`Enter ${f.display_name}...`}
                        className="w-full px-4 py-2.5 rounded-xl bg-slate-900 border border-slate-700 text-sm text-slate-100 focus:border-brand-500"
                        required={f.required}
                      />
                    )}

                    {f.help_text && <p className="text-[11px] text-slate-400 pl-1">{f.help_text}</p>}
                  </div>
                );
              })}

              {/* MODAL ACTIONS */}
              <div className="pt-4 border-t border-slate-800 flex justify-end space-x-3 sticky bottom-0 bg-slate-900/90 py-2 backdrop-blur-md rounded-b-xl">
                <button type="button" onClick={onClose} disabled={saving} className="px-5 py-2.5 rounded-xl text-sm font-medium text-slate-300 hover:bg-slate-800 hover:text-white transition">
                  Cancel
                </button>
                <button
                  type="submit"
                  disabled={saving}
                  className="px-6 py-2.5 bg-gradient-to-r from-brand-500 to-emerald-400 hover:from-brand-600 hover:to-emerald-500 disabled:opacity-50 text-black text-sm font-bold rounded-xl transition shadow-lg shadow-brand-500/25 flex items-center space-x-2"
                >
                  {saving ? (
                    <>
                      <span className="animate-spin inline-block mr-1">⏳</span>
                      <span>{record ? 'Updating Record...' : 'Creating Record...'}</span>
                    </>
                  ) : (
                    <>
                      <span>{record ? 'Update Record' : 'Create Record'}</span>
                      <span>&rarr;</span>
                    </>
                  )}
                </button>
              </div>
            </form>
          </div>
        </div>
      );
    }

    // AUDIT LOGS & TIME TRAVEL VIEW
    function AuditLogsView({ showToast }) {
      const [logs, setLogs] = useState([]);
      const [loading, setLoading] = useState(true);

      const fetchLogs = useCallback(async () => {
        setLoading(true);
        try {
          const res = await apiRequest('/api/d/audit-logs?limit=50');
          if (res.success) {
            setLogs(res.data);
          }
        } catch (e) {
          showToast('Failed to load audit logs', true);
        } finally {
          setLoading(false);
        }
      }, [showToast]);

      useEffect(() => {
        fetchLogs();
      }, [fetchLogs]);

      const handleRollback = async (logId) => {
        if (!confirm(`Are you sure you want to revert to snapshot at audit log #${logId}?`)) return;
        try {
          const res = await apiRequest(`/api/d/rollback/${logId}`, { method: 'POST' });
          if (res.success) {
            showToast('Time-travel rollback successful!');
            fetchLogs();
          } else {
            showToast(res.message || 'Rollback failed', true);
          }
        } catch (e) {
          showToast('Rollback failed', true);
        }
      };

      return (
        <div className="glass p-6 rounded-2xl border border-slate-800 space-y-6">
          <div className="overflow-x-auto custom-scrollbar border border-slate-800 rounded-xl">
            <table className="w-full text-left text-sm">
              <thead className="bg-slate-900 text-xs uppercase tracking-wider text-slate-400 border-b border-slate-800">
                <tr>
                  <th className="px-4 py-3">ID</th>
                  <th className="px-4 py-3">Model</th>
                  <th className="px-4 py-3">Action</th>
                  <th className="px-4 py-3">User</th>
                  <th className="px-4 py-3">Timestamp</th>
                  <th className="px-4 py-3">Changes / Snapshot</th>
                  <th className="px-4 py-3 text-right">Time-Travel</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-slate-800/60 font-mono text-xs">
                {loading ? (
                  <tr><td colSpan="100" className="p-8 text-center text-slate-500 font-sans">Loading audit trail...</td></tr>
                ) : logs.length === 0 ? (
                  <tr><td colSpan="100" className="p-8 text-center text-slate-500 font-sans">No audit events logged yet.</td></tr>
                ) : (
                  logs.map(log => (
                    <tr key={log.id} className="hover:bg-slate-800/40 transition">
                      <td className="px-4 py-3 text-slate-400">#{log.id}</td>
                      <td className="px-4 py-3 font-semibold text-white font-sans">
                        {log.model_name} <span className="text-slate-500 text-xs">(#{log.record_id})</span>
                      </td>
                      <td className="px-4 py-3 font-sans">
                        <span className={`px-2 py-0.5 rounded text-[10px] font-bold ${
                          log.action === 'CREATE' ? 'bg-emerald-500/20 text-emerald-400' :
                          log.action === 'UPDATE' ? 'bg-indigo-500/20 text-indigo-400' :
                          log.action === 'DELETE' ? 'bg-rose-500/20 text-rose-400' : 'bg-amber-500/20 text-amber-400'
                        }`}>{log.action}</span>
                      </td>
                      <td className="px-4 py-3 text-slate-300 font-sans">{log.username || 'system'}</td>
                      <td className="px-4 py-3 text-slate-400 text-[11px]">{log.created_at}</td>
                      <td className="px-4 py-3 text-slate-400 truncate max-w-xs">{JSON.stringify(log.changes)}</td>
                      <td className="px-4 py-3 text-right font-sans">
                        {log.action === 'DELETE' || log.action === 'UPDATE' ? (
                          <button
                            onClick={() => handleRollback(log.id)}
                            className="px-3 py-1 bg-indigo-600 hover:bg-indigo-500 text-white font-bold text-xs rounded-lg transition shadow-md"
                          >
                            Rollback ↺
                          </button>
                        ) : (
                          <span className="text-slate-600">-</span>
                        )}
                      </td>
                    </tr>
                  ))
                )}
              </tbody>
            </table>
          </div>
        </div>
      );
    }

    // APPROVAL QUEUE VIEW WITH AI RISK SCORING
    function ApprovalQueueView({ showToast, onApprovalsUpdated }) {
      const [approvals, setApprovals] = useState([]);
      const [loading, setLoading] = useState(true);

      const fetchApprovals = useCallback(async () => {
        setLoading(true);
        try {
          const res = await apiRequest('/api/d/approvals');
          if (res.success) {
            setApprovals(res.data);
            onApprovalsUpdated();
          }
        } catch (e) {
          showToast('Failed to load approvals', true);
        } finally {
          setLoading(false);
        }
      }, [showToast, onApprovalsUpdated]);

      useEffect(() => {
        fetchApprovals();
      }, [fetchApprovals]);

      const handleApprove = async (id) => {
        try {
          const res = await apiRequest(`/api/d/approvals/${id}/approve`, { method: 'POST' });
          if (res.success) {
            showToast('Change approved and committed to database!');
            fetchApprovals();
          } else {
            showToast(res.message || 'Approval failed', true);
          }
        } catch (e) {
          showToast('Approval error', true);
        }
      };

      const handleReject = async (id) => {
        try {
          const res = await apiRequest(`/api/d/approvals/${id}/reject`, { method: 'POST' });
          if (res.success) {
            showToast('Change request rejected');
            fetchApprovals();
          } else {
            showToast(res.message || 'Rejection failed', true);
          }
        } catch (e) {
          showToast('Rejection error', true);
        }
      };

      return (
        <div className="glass p-6 rounded-2xl border border-slate-800 space-y-6">
          <div className="overflow-x-auto custom-scrollbar border border-slate-800 rounded-xl">
            <table className="w-full text-left text-sm">
              <thead className="bg-slate-900 text-xs uppercase tracking-wider text-slate-400 border-b border-slate-800">
                <tr>
                  <th className="px-4 py-3">ID</th>
                  <th className="px-4 py-3">Model</th>
                  <th className="px-4 py-3">Field</th>
                  <th className="px-4 py-3">Old Value</th>
                  <th className="px-4 py-3">New Value</th>
                  <th className="px-4 py-3">AI Risk Assessment</th>
                  <th className="px-4 py-3 text-right">Decision</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-slate-800/60 font-mono text-xs">
                {loading ? (
                  <tr><td colSpan="100" className="p-8 text-center text-slate-500 font-sans">Loading pending approvals...</td></tr>
                ) : approvals.length === 0 ? (
                  <tr><td colSpan="100" className="p-8 text-center text-slate-500 font-sans">No pending approvals! All changes are up to date.</td></tr>
                ) : (
                  approvals.map(app => {
                    const aiRisk = app.ai_risk || { risk_level: 'Low', recommendation: 'Standard change' };
                    const riskColor = aiRisk.risk_level === 'Low' ? 'bg-emerald-500/20 text-emerald-300' :
                      (aiRisk.risk_level === 'Medium' ? 'bg-amber-500/20 text-amber-300' : 'bg-rose-500/20 text-rose-300');

                    return (
                      <tr key={app.id} className="hover:bg-slate-800/40 transition">
                        <td className="px-4 py-3 text-slate-400">#{app.id}</td>
                        <td className="px-4 py-3 font-semibold text-white font-sans">{app.model_name} (#{app.record_id})</td>
                        <td className="px-4 py-3 text-amber-300 font-semibold">{app.field_name}</td>
                        <td className="px-4 py-3 text-rose-400 line-through">{app.old_value || 'null'}</td>
                        <td className="px-4 py-3 text-emerald-400 font-bold">{app.new_value}</td>
                        <td className="px-4 py-3 font-sans">
                          <div className="space-y-1">
                            <span className={`px-2 py-0.5 rounded text-[10px] font-bold ${riskColor}`}>
                              {aiRisk.risk_level} RISK
                            </span>
                            <div className="text-[11px] text-slate-400 truncate max-w-xs">{aiRisk.recommendation}</div>
                          </div>
                        </td>
                        <td className="px-4 py-3 text-right space-x-2 font-sans">
                          <button onClick={() => handleApprove(app.id)} className="px-3 py-1 bg-emerald-600 hover:bg-emerald-500 text-white font-bold text-xs rounded-lg transition">Approve</button>
                          <button onClick={() => handleReject(app.id)} className="px-3 py-1 bg-rose-600 hover:bg-rose-500 text-white font-bold text-xs rounded-lg transition">Reject</button>
                        </td>
                      </tr>
                    );
                  })
                )}
              </tbody>
            </table>
          </div>
        </div>
      );
    }

    // FRONTEND ECOSYSTEM HUB (REACT, VUE, ANGULAR)
    function ReactSdkView() {
      const [activeFramework, setActiveFramework] = useState('react');

      const reactCode = `// ⚛️ React 18+ / Next.js Example
import { OxideProvider, OxideClient, useOxideQuery, useOxideMutation } from './oxide-react';

const client = new OxideClient('http://localhost:8080');

export default function App() {
  return (
    <OxideProvider client={client}>
      <ProductCatalog />
    </OxideProvider>
  );
}

function ProductCatalog() {
  // Query with auto-filtering, sorting, and pagination
  const { data: products, total, isLoading } = useOxideQuery('Product', {
    order: '-created_at',
    filters: { in_stock: true, price__gte: 50 },
    limit: 10,
  });

  const { create, remove } = useOxideMutation('Product');

  if (isLoading) return <div>Loading from Rust backend...</div>;

  return (
    <div>
      <h2>Products ({total})</h2>
      {products.map(p => (
        <div key={p.id}>
          <h3>{p.name} - \${p.price}</h3>
          <button onClick={() => remove(p.id)}>Delete</button>
        </div>
      ))}
    </div>
  );
}`;

      const vueCode = `<!-- 🟢 Vue 3 Composition API & Nuxt 3 Example -->
<script setup lang="ts">
import { useOxideVueQuery, useOxideVueMutation } from './oxide-vue';

// Reactive query with Vue ref & computed signals
const { data: products, total, isLoading, refetch } = useOxideVueQuery('Product', {
  order: '-created_at',
  filters: { in_stock: true, price__gte: 50 },
  limit: 10,
});

const { remove } = useOxideVueMutation('Product');
</script>

<template>
  <div>
    <h2>Products ({{ total }})</h2>
    <div v-if="isLoading">Loading from Rust backend...</div>
    <div v-else>
      <div v-for="p in products" :key="p.id" class="product-card">
        <h3>{{ p.name }} - \${{ p.price }}</h3>
        <button @click="remove(p.id)">Delete</button>
      </div>
    </div>
  </div>
</template>`;

      const angularCode = `// 🅰️ Angular 17/18 Standalone Component (Signals & Injectable)
import { Component, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { OxideService } from './oxide-angular';

@Component({
  selector: 'app-product-list',
  standalone: true,
  imports: [CommonModule],
  template: \`
    <h2>Products ({{ query.total() }})</h2>
    <div *ngIf="query.isLoading()">Loading from Rust backend...</div>
    <div *ngFor="let p of query.data()" class="product-card">
      <h3>{{ p.name }} - \${{ p.price }}</h3>
      <button (click)="deleteProduct(p.id)">Delete</button>
    </div>
  \`,
})
export class ProductListComponent {
  private oxide = inject(OxideService);

  // Angular Signal Query
  readonly query = this.oxide.createSignalQuery('Product', {
    order: '-created_at',
    filters: { in_stock: true },
  });

  async deleteProduct(id: number) {
    await this.oxide.delete('Product', id);
    this.query.refetch();
  }
}`;

      return (
        <div className="space-y-6">
          <div className="glass p-6 rounded-2xl border border-slate-800">
            <div className="flex items-center justify-between mb-6">
              <div>
                <h3 className="text-xl font-bold text-white flex items-center space-x-2">
                  <span>🚀 Frontend Ecosystem SDK Hub</span>
                  <span className="text-xs bg-brand-500/20 text-brand-300 border border-brand-500/30 px-2 py-0.5 rounded-full font-mono font-bold">
                    React &bull; Vue &bull; Angular
                  </span>
                </h3>
                <p className="text-xs text-slate-400 mt-1">Oxide_CG serves auto-generated TypeScript SDKs with native hooks, composables, and signals</p>
              </div>

              {/* DOWNLOAD LINKS */}
              <div className="flex items-center space-x-2">
                <a href="/api/sdk/react.ts" target="_blank" className="px-3 py-1.5 bg-react-500/10 hover:bg-react-500/20 text-react-400 border border-react-500/30 rounded-lg text-xs font-mono font-semibold transition">
                  ⚛️ react.ts
                </a>
                <a href="/api/sdk/vue.ts" target="_blank" className="px-3 py-1.5 bg-emerald-500/10 hover:bg-emerald-500/20 text-emerald-400 border border-emerald-500/30 rounded-lg text-xs font-mono font-semibold transition">
                  🟢 vue.ts
                </a>
                <a href="/api/sdk/angular.ts" target="_blank" className="px-3 py-1.5 bg-rose-500/10 hover:bg-rose-500/20 text-rose-400 border border-rose-500/30 rounded-lg text-xs font-mono font-semibold transition">
                  🅰️ angular.ts
                </a>
              </div>
            </div>

            {/* FRAMEWORK SELECTOR TABS */}
            <div className="flex space-x-2 border-b border-slate-800 pb-3 mb-4">
              <button
                onClick={() => setActiveFramework('react')}
                className={`px-4 py-2 rounded-xl text-xs font-bold transition flex items-center space-x-2 ${
                  activeFramework === 'react' ? 'bg-react-500/20 text-react-300 border border-react-500/40' : 'text-slate-400 hover:bg-slate-800'
                }`}
              >
                <span>⚛️</span>
                <span>React 18/19 & Next.js</span>
              </button>
              <button
                onClick={() => setActiveFramework('vue')}
                className={`px-4 py-2 rounded-xl text-xs font-bold transition flex items-center space-x-2 ${
                  activeFramework === 'vue' ? 'bg-emerald-500/20 text-emerald-300 border border-emerald-500/40' : 'text-slate-400 hover:bg-slate-800'
                }`}
              >
                <span>🟢</span>
                <span>Vue 3 Composition & Nuxt</span>
              </button>
              <button
                onClick={() => setActiveFramework('angular')}
                className={`px-4 py-2 rounded-xl text-xs font-bold transition flex items-center space-x-2 ${
                  activeFramework === 'angular' ? 'bg-rose-500/20 text-rose-300 border border-rose-500/40' : 'text-slate-400 hover:bg-slate-800'
                }`}
              >
                <span>🅰️</span>
                <span>Angular 17+ Signals & RxJS</span>
              </button>
            </div>

            {/* CODE VIEWER */}
            <div className="bg-slate-950 p-5 rounded-xl border border-slate-800 font-mono text-xs text-slate-300 overflow-x-auto leading-relaxed custom-scrollbar">
              <pre>{activeFramework === 'react' ? reactCode : (activeFramework === 'vue' ? vueCode : angularCode)}</pre>
            </div>
          </div>
        </div>
      );
    }

    // RENDER REACT ROOT
    const root = ReactDOM.createRoot(document.getElementById('root'));
    root.render(<App />);
  </script>
</body>
</html>"#;

    raw_html.replace("__SITE_NAME__", site_name)
}
