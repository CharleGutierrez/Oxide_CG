/// Auto-generated TypeScript / Vue 3 Composition API SDK for Oxide_CG backend.
pub fn generate_vue_sdk(base_url: &str) -> String {
    format!(
        r#"/**
 * 🟢 Oxide_CG Vue 3 Client SDK (Composition API & Pinia Ready)
 * Auto-generated Vue Composables & Client for Oxide_CG backend.
 * Compatible with Vue 3.4+, Nuxt 3, Vite, and Pinia.
 */

import {{ ref, computed, watch, onMounted, inject, provide, type Ref, type ComputedRef }} from 'vue';

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

export interface VueQueryResult<T = any> {{
  data: Ref<T[]>;
  total: Ref<number>;
  limit: Ref<number>;
  offset: Ref<number>;
  isLoading: Ref<boolean>;
  error: Ref<Error | null>;
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

  async rollback(logId: number): Promise<{{ success: boolean; message: string }}> {{
    return this.request(`/api/d/rollback/${{logId}}`, {{ method: 'POST' }});
  }}
}}

// Vue 3 Dependency Injection Symbol
const OxideClientSymbol = Symbol('OxideClient');

export function provideOxide(client: OxideClient) {{
  provide(OxideClientSymbol, client);
}}

export function useOxide(): OxideClient {{
  const client = inject<OxideClient>(OxideClientSymbol);
  if (!client) {{
    return new OxideClient();
  }}
  return client;
}}

/**
 * Vue 3 Reactive Query Composable
 */
export function useOxideVueQuery<T = any>(model: string, options: QueryOptions = {{}}): VueQueryResult<T> {{
  const client = useOxide();
  const data = ref<T[]>([]) as Ref<T[]>;
  const total = ref(0);
  const limit = ref(options.limit || 50);
  const offset = ref(options.offset || 0);
  const isLoading = ref(true);
  const error = ref<Error | null>(null);

  const fetchRecords = async () => {{
    isLoading.value = true;
    error.value = null;
    try {{
      const res = await client.list<T>(model, {{
        ...options,
        limit: limit.value,
        offset: offset.value,
      }});
      if (res.success) {{
        data.value = res.data;
        total.value = res.total;
        limit.value = res.limit;
        offset.value = res.offset;
      }}
    }} catch (err: any) {{
      error.value = err;
    }} finally {{
      isLoading.value = false;
    }}
  }};

  onMounted(fetchRecords);

  return {{
    data,
    total,
    limit,
    offset,
    isLoading,
    error,
    refetch: fetchRecords,
  }};
}}

/**
 * Vue 3 Mutation Composable (Create, Update, Delete)
 */
export function useOxideVueMutation<T = any>(model: string) {{
  const client = useOxide();
  const isMutating = ref(false);
  const error = ref<Error | null>(null);

  const create = async (payload: Partial<T>) => {{
    isMutating.value = true;
    error.value = null;
    try {{
      return await client.create<T>(model, payload);
    }} catch (err: any) {{
      error.value = err;
      throw err;
    }} finally {{
      isMutating.value = false;
    }}
  }};

  const update = async (id: number, payload: Partial<T>) => {{
    isMutating.value = true;
    error.value = null;
    try {{
      return await client.update<T>(model, id, payload);
    }} catch (err: any) {{
      error.value = err;
      throw err;
    }} finally {{
      isMutating.value = false;
    }}
  }};

  const remove = async (id: number) => {{
    isMutating.value = true;
    error.value = null;
    try {{
      return await client.delete(model, id);
    }} catch (err: any) {{
      error.value = err;
      throw err;
    }} finally {{
      isMutating.value = false;
    }}
  }};

  return {{ create, update, remove, isMutating, error }};
}}
"#,
        base_url = base_url
    )
}
