/// Auto-generated TypeScript / Angular 17+ Service & Signal SDK for Oxide_CG backend.
pub fn generate_angular_sdk(base_url: &str) -> String {
    format!(
        r#"/**
 * 🅰️ Oxide_CG Angular 17/18 Client SDK (Signals & RxJS Ready)
 * Auto-generated Angular Injectable Service & Signal Helpers.
 * Compatible with Angular 17+, Standalone Components, and RxJS.
 */

import {{ Injectable, signal, WritableSignal, computed, Signal }} from '@angular/core';

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

export interface OxideListResponse<T = any> {{
  success: boolean;
  total: number;
  limit: number;
  offset: number;
  data: T[];
}}

export interface OxideSingleResponse<T = any> {{
  success: boolean;
  data: T;
}}

@Injectable({{
  providedIn: 'root',
}})
export class OxideService {{
  private baseUrl = '{base_url}'.replace(/\/$/, '');
  private token: string | null = null;

  // Global Auth Signal
  readonly currentUser: WritableSignal<OxideUser | null> = signal(null);
  readonly isAuthenticated: Signal<boolean> = computed(() => this.currentUser() !== null);

  constructor() {{}}

  setToken(token: string | null) {{
    this.token = token;
  }}

  private async request<T = any>(path: string, options: RequestInit = {{}}): Promise<T> {{
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
  async login(username: string, password: string): Promise<any> {{
    const res = await this.request('/api/auth/login', {{
      method: 'POST',
      body: JSON.stringify({{ username, password }}),
    }});
    if (res.session) {{
      this.setToken(res.session.token);
      this.currentUser.set({{
        id: res.session.user_id,
        username: res.session.username,
        email: '',
        role: res.session.role,
        is_active: true,
      }});
    }}
    return res;
  }}

  async logout(): Promise<void> {{
    await this.request('/api/auth/logout', {{ method: 'POST' }});
    this.setToken(null);
    this.currentUser.set(null);
  }}

  async getMe(): Promise<{{ success: boolean; user: OxideUser }}> {{
    const res = await this.request<{{ success: boolean; user: OxideUser }}>('/api/auth/me');
    if (res.success && res.user) {{
      this.currentUser.set(res.user);
    }}
    return res;
  }}

  // Model CRUD operations
  async list<T = any>(model: string, options: QueryOptions = {{}}): Promise<OxideListResponse<T>> {{
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
    return this.request<OxideListResponse<T>>(`/api/d/${{model.toLowerCase()}}${{query}}`);
  }}

  async get<T = any>(model: string, id: number): Promise<OxideSingleResponse<T>> {{
    return this.request<OxideSingleResponse<T>>(`/api/d/${{model.toLowerCase()}}/${{id}}`);
  }}

  async create<T = any>(model: string, payload: Partial<T>): Promise<OxideSingleResponse<T>> {{
    return this.request<OxideSingleResponse<T>>(`/api/d/${{model.toLowerCase()}}`, {{
      method: 'POST',
      body: JSON.stringify(payload),
    }});
  }}

  async update<T = any>(model: string, id: number, payload: Partial<T>): Promise<OxideSingleResponse<T>> {{
    return this.request<OxideSingleResponse<T>>(`/api/d/${{model.toLowerCase()}}/${{id}}`, {{
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

  /**
   * Helper to create an Angular Signal Query resource
   */
  createSignalQuery<T = any>(model: string, initialOptions: QueryOptions = {{}}) {{
    const data: WritableSignal<T[]> = signal([]);
    const total: WritableSignal<number> = signal(0);
    const isLoading: WritableSignal<boolean> = signal(false);
    const error: WritableSignal<Error | null> = signal(null);

    const execute = async (options: QueryOptions = initialOptions) => {{
      isLoading.set(true);
      error.set(null);
      try {{
        const res = await this.list<T>(model, options);
        if (res.success) {{
          data.set(res.data);
          total.set(res.total);
        }}
      }} catch (err: any) {{
        error.set(err);
      }} finally {{
        isLoading.set(false);
      }}
    }};

    // Initial fetch
    execute();

    return {{
      data: data.asReadonly(),
      total: total.asReadonly(),
      isLoading: isLoading.asReadonly(),
      error: error.asReadonly(),
      refetch: execute,
    }};
  }}
}}
"#,
        base_url = base_url
    )
}
