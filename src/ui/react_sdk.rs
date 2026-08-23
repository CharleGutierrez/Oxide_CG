/// Auto-generated TypeScript / React SDK for frontend developers connecting to Oxide_CG dAPI.
pub fn generate_react_sdk(base_url: &str) -> String {
    format!(
        r#"/**
 * ⚡ Oxide_CG React Client SDK
 * Auto-generated React Hooks & API Client for Oxide_CG backend.
 * Compatible with React 18+, Next.js, Remix, and Vite.
 */

import React, {{ createContext, useContext, useState, useEffect, useCallback }} from 'react';

export interface OxideUser {{
  id: number;
  username: string;
  email: string;
  role: 'Admin' | 'Manager' | 'Editor' | 'Viewer';
  is_active: boolean;
}}

export interface QueryOptions {{
  limit?: number;
  offset?: number;
  order?: string;
  search?: string;
  filters?: Record<string, string | number | boolean>;
}}

export interface QueryResult<T = any> {{
  data: T[];
  total: number;
  limit: number;
  offset: number;
  isLoading: boolean;
  error: Error | null;
  refetch: () => Promise<void>;
}}

export class OxideClient {{
  private baseUrl: string;
  private token: string | null = null;

  constructor(baseUrl: string = '{base_url}') {{
    this.baseUrl = baseUrl.replace(/\/$/, '');
  }}

  setToken(token: string | null) {{
    this.token = token;
  }}

  async request<T = any>(path: string, options: RequestInit = {{}}): Promise<T> {{
    const headers: Record<string, string> = {{
      'Content-Type': 'application/json',
      ...(options.headers as Record<string, string> || {{}}),
    }};

    if (this.token) {{
      headers['Authorization'] = `Bearer ${{this.token}}`;
    }}

    const res = await fetch(`${{this.baseUrl}}${{path}}`, {{
      ...options,
      headers,
      credentials: 'include',
    }});

    if (!res.ok) {{
      const errBody = await res.json().catch(() => ({{ message: res.statusText }}));
      throw new Error(errBody.message || `Request failed with status ${{res.status}}`);
    }}

    return res.json();
  }}

  // Auth methods
  async login(username: string, password: string) {{
    const res = await this.request('/api/auth/login', {{
      method: 'POST',
      body: JSON.stringify({{ username, password }}),
    }});
    if (res.session?.token) {{
      this.setToken(res.session.token);
    }}
    return res;
  }}

  async logout() {{
    await this.request('/api/auth/logout', {{ method: 'POST' }});
    this.setToken(null);
  }}

  async getMe(): Promise<{{ success: boolean; user: OxideUser }}> {{
    return this.request('/api/auth/me');
  }}

  // Model CRUD methods
  async list<T = any>(model: string, options: QueryOptions = {{}}): Promise<{{ success: boolean; total: number; limit: number; offset: number; data: T[] }}> {{
    const params = new URLSearchParams();
    if (options.limit) params.set('$limit', options.limit.toString());
    if (options.offset) params.set('$offset', options.offset.toString());
    if (options.order) params.set('$order', options.order);
    if (options.search) params.set('$search', options.search);

    if (options.filters) {{
      for (const [k, v] of Object.entries(options.filters)) {{
        params.set(k, String(v));
      }}
    }}

    const query = params.toString() ? `?${{params.toString()}}` : '';
    return this.request(`/api/d/${{model.toLowerCase()}}${{query}}`);
  }}

  async get<T = any>(model: string, id: number): Promise<{{ success: boolean; data: T }}> {{
    return this.request(`/api/d/${{model.toLowerCase()}}/${{id}}`);
  }}

  async create<T = any>(model: string, payload: Partial<T>): Promise<{{ success: boolean; data: T }}> {{
    return this.request(`/api/d/${{model.toLowerCase()}}`, {{
      method: 'POST',
      body: JSON.stringify(payload),
    }});
  }}

  async update<T = any>(model: string, id: number, payload: Partial<T>): Promise<{{ success: boolean; data: T }}> {{
    return this.request(`/api/d/${{model.toLowerCase()}}/${{id}}`, {{
      method: 'PUT',
      body: JSON.stringify(payload),
    }});
  }}

  async delete(model: string, id: number): Promise<{{ success: boolean; message: string }}> {{
    return this.request(`/api/d/${{model.toLowerCase()}}/${{id}}`, {{
      method: 'DELETE',
    }});
  }}

  // Audit & Time-Travel
  async rollback(logId: number): Promise<{{ success: boolean; message: string }}> {{
    return this.request(`/api/d/rollback/${{logId}}`, {{ method: 'POST' }});
  }}
}}

// React Context & Hooks
const OxideContext = createContext<OxideClient | null>(null);

export const OxideProvider: React.FC<{{ client: OxideClient; children: React.ReactNode }}> = ({{ client, children }}) => {{
  return <OxideContext.Provider value={{client}}>{{children}}</OxideContext.Provider>;
}};

export function useOxideClient(): OxideClient {{
  const client = useContext(OxideContext);
  if (!client) {{
    throw new Error('useOxideClient must be used within an OxideProvider');
  }}
  return client;
}}

export function useOxideQuery<T = any>(model: string, options: QueryOptions = {{}}): QueryResult<T> {{
  const client = useOxideClient();
  const [data, setData] = useState<T[]>([]);
  const [total, setTotal] = useState(0);
  const [limit, setLimit] = useState(options.limit || 50);
  const [offset, setOffset] = useState(options.offset || 0);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  const fetchRecords = useCallback(async () => {{
    setIsLoading(true);
    setError(null);
    try {{
      const res = await client.list<T>(model, options);
      if (res.success) {{
        setData(res.data);
        setTotal(res.total);
        setLimit(res.limit);
        setOffset(res.offset);
      }}
    }} catch (err: any) {{
      setError(err);
    }} finally {{
      setIsLoading(false);
    }}
  }}, [client, model, JSON.stringify(options)]);

  useEffect(() => {{
    fetchRecords();
  }}, [fetchRecords]);

  return {{ data, total, limit, offset, isLoading, error, refetch: fetchRecords }};
}}

export function useOxideMutation<T = any>(model: string) {{
  const client = useOxideClient();

  const create = useCallback((payload: Partial<T>) => client.create<T>(model, payload), [client, model]);
  const update = useCallback((id: number, payload: Partial<T>) => client.update<T>(model, id, payload), [client, model]);
  const remove = useCallback((id: number) => client.delete(model, id), [client, model]);

  return {{ create, update, remove }};
}}
"#,
        base_url = base_url
    )
}
