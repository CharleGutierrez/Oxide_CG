# 📚 Comprehensive Tutorial: Building with Oxide_CG

Welcome to the complete, step-by-step developer guide for **Oxide_CG** — the ultra-fast Rust web framework and auto-admin engine with Multi-Database support, an AI Tuner, and native React, Vue 3, and Angular integration.

---

## 📑 Table of Contents
1. [Prerequisites & Quick Setup](#1-prerequisites--quick-setup)
2. [Declaring Models & Field Types](#2-declaring-models--field-types)
3. [Connecting Databases (SQLite, PostgreSQL, MySQL)](#3-connecting-databases)
4. [Using the RESTful Data API (dAPI) & Query Filters](#4-using-the-restful-data-api-dapi)
5. [Frontend Integration (React, Vue 3, Angular)](#5-frontend-integration)
6. [Working with the AI Tuner & Decision Engine](#6-working-with-the-ai-tuner--decision-engine)
7. [Audit Trails & 1-Click Time-Travel Rollback](#7-audit-trails--time-travel-rollback)
8. [Approval Workflows for Sensitive Data](#8-approval-workflows)
9. [Lifecycle Hooks & Event Bus Extensions](#9-lifecycle-hooks--event-bus)
10. [Production Deployment & Kubernetes Setup](#10-production-deployment)

---

## 1. Prerequisites & Quick Setup

Ensure you have Rust (version 1.75+) installed:

```bash
rustc --version
cargo --version
```

### Creating Your Project
Create a new binary application and add `oxide_cg`:

```bash
cargo new my_app --bin
cd my_app
```

In your `Cargo.toml`:

```toml
[dependencies]
oxide_cg = "0.1.0"
tokio = { version = "1", features = ["full"] }
serde_json = "1.0"
async-trait = "0.1"
```

---

## 2. Declaring Models & Field Types

Models in Oxide_CG are declarative schema definitions that map to database tables, admin UI controls, and REST endpoints:

```rust
use oxide_cg::prelude::*;

fn build_product_model() -> ModelSchema {
    ModelSchema::new("Product")
        .category("E-Commerce")
        .icon("shopping-bag")
        .description("Store catalog items")
        // Required & Searchable String Field
        .field(Field::string("name").required().searchable())
        // Unique SKU
        .field(Field::string("sku").required().unique().searchable())
        // Currency / Money Field ($ USD)
        .field(Field::money("price", "USD").required().filterable(true))
        // Sensitive Field: Triggers AI Risk Scoring & Approval Queue
        .field(Field::float("discount_percent").requires_approval().help("Discounts >20% need review"))
        // Visual Progress Bar (Max 1000 items)
        .field(Field::progress_bar("stock_quantity", 1000.0, "#10b981"))
        // Rich Text HTML / Markdown Field
        .field(Field::html("description"))
        // Enum Dropdown
        .field(Field::r#enum("status", vec!["Draft", "Published", "Archived"]))
        // Foreign Key Relation to Category model
        .field(Field::foreign_key("category_id", "Category"))
        // Boolean Toggle with default
        .field(Field::boolean("is_featured").default_value(serde_json::json!(false)))
        // Automatically adds created_at and updated_at timestamps
        .with_timestamps()
}
```

### Supported Field Types & Helpers
| Helper | Description | UI Widget |
| :--- | :--- | :--- |
| `Field::string(name)` | Standard text field | Text input |
| `Field::integer(name)` | 64-bit integer | Number input |
| `Field::float(name)` | 64-bit floating point | Decimal input |
| `Field::boolean(name)` | Boolean flag | Toggle switch |
| `Field::datetime(name)` | ISO 8601 Timestamp | DateTime picker |
| `Field::email(name)` | Email string with validation | Email input |
| `Field::password(name)` | Salted & hashed secret | Password input (masked in lists) |
| `Field::money(name, curr)` | Formatted currency value | Number with currency badge |
| `Field::progress_bar(name, max, col)` | Progress / Rating meter | Colored progress bar |
| `Field::html(name)` | Rich text / HTML content | Textarea / Code editor |
| `Field::markdown(name)` | Markdown formatted text | Markdown previewer |
| `Field::r#enum(name, choices)` | Constrained set of choices | Dropdown select |
| `Field::foreign_key(name, target)` | Relational link to target model | Relational picker |
| `Field::json(name)` | Dynamic JSON tree | JSON editor |

---

## 3. Connecting Databases

Oxide_CG automatically adapts its SQL dialect based on the connection string:

### SQLite (Embedded / Local / Edge)
```rust
OxideCGApp::new()
    .database("sqlite://app.db?mode=rwc")
    .run()
    .await?;
```

### PostgreSQL (Enterprise Multi-Node)
```rust
OxideCGApp::new()
    .database("postgres://postgres:password@localhost:5432/my_production_db")
    .max_connections(50)
    .run()
    .await?;
```

### MySQL / MariaDB (Enterprise InnoDB)
```rust
OxideCGApp::new()
    .database("mysql://root:password@localhost:3306/my_production_db")
    .max_connections(50)
    .run()
    .await?;
```

---

## 4. Using the RESTful Data API (dAPI)

Every registered model receives full REST endpoints under `/api/d/{model}`:

### Filtering, Searching, and Pagination
```bash
# Get page 1 of published products priced above $50, sorted by newest
GET /api/d/product?status=Published&price__gte=50&$order=-created_at&$limit=10&$offset=0

# Search all searchable fields for "mechanical"
GET /api/d/product?$search=mechanical

# Find products in specific categories
GET /api/d/product?category_id__in=1,2,5
```

### CRUD Operations
```bash
# Create a new record
POST /api/d/product
Content-Type: application/json
{
  "name": "Rust Mechanical Keyboard",
  "sku": "KB-RUST-01",
  "price": 149.99,
  "stock_quantity": 85,
  "status": "Published"
}

# Update a record
PUT /api/d/product/1
Content-Type: application/json
{
  "price": 129.99
}

# Delete a record (Automatically creates time-travel backup snapshot)
DELETE /api/d/product/1
```

---

## 5. Frontend Integration

Oxide_CG serves auto-generated client SDKs directly from your running server:

### ⚛️ React 18 / Next.js (`/api/sdk/react.ts`)
```tsx
import React from 'react';
import { OxideProvider, OxideClient, useOxideQuery, useOxideMutation } from './oxide-react';

const client = new OxideClient('http://localhost:8080');

export default function App() {
  return (
    <OxideProvider client={client}>
      <ProductsView />
    </OxideProvider>
  );
}

function ProductsView() {
  const { data: products, total, isLoading, refetch } = useOxideQuery('Product', {
    order: '-created_at',
    filters: { status: 'Published', price__gte: 20 },
    limit: 10,
  });

  const { remove } = useOxideMutation('Product');

  if (isLoading) return <div>Loading from Oxide_CG...</div>;

  return (
    <div>
      <h1>Catalog ({total})</h1>
      {products.map(p => (
        <div key={p.id}>
          <h3>{p.name} — ${p.price}</h3>
          <button onClick={() => remove(p.id)}>Delete</button>
        </div>
      ))}
    </div>
  );
}
```

### 🟢 Vue 3 Composition API & Nuxt 3 (`/api/sdk/vue.ts`)
```vue
<script setup lang="ts">
import { useOxideVueQuery, useOxideVueMutation } from './oxide-vue';

const { data: products, total, isLoading, refetch } = useOxideVueQuery('Product', {
  order: '-created_at',
  filters: { in_stock: true },
});

const { remove } = useOxideVueMutation('Product');
</script>

<template>
  <div>
    <h2>Total Products: {{ total }}</h2>
    <div v-if="isLoading">Loading...</div>
    <div v-else>
      <div v-for="p in products" :key="p.id">
        <h3>{{ p.name }} — ${{ p.price }}</h3>
        <button @click="remove(p.id)">Delete</button>
      </div>
    </div>
  </div>
</template>
```

### 🅰️ Angular 17+ Standalone & Signals (`/api/sdk/angular.ts`)
```typescript
import { Component, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { OxideService } from './oxide-angular';

@Component({
  selector: 'app-products',
  standalone: true,
  imports: [CommonModule],
  template: `
    <h2>Products ({{ query.total() }})</h2>
    <div *ngIf="query.isLoading()">Loading from Oxide_CG...</div>
    <div *ngFor="let p of query.data()">
      <h3>{{ p.name }} - \${{ p.price }}</h3>
      <button (click)="deleteItem(p.id)">Delete</button>
    </div>
  `,
})
export class ProductsComponent {
  private oxide = inject(OxideService);

  readonly query = this.oxide.createSignalQuery('Product', {
    order: '-created_at',
    filters: { status: 'Published' },
  });

  async deleteItem(id: number) {
    await this.oxide.delete('Product', id);
    this.query.refetch();
  }
}
```

---

## 6. Working with the AI Tuner & Decision Engine

### A. Checking AI Latency Telemetry (`GET /api/ai/report`)
The AI Tuner analyzes runtime queries and reports median (p50), 95th percentile, and p99 tail latencies:

```bash
curl http://localhost:8080/api/ai/report
```

### B. Auto-Applying Recommended Indexes
When queries filter columns without indexes, the AI Tuner recommends and can auto-apply indexes:

```bash
curl -X POST "http://localhost:8080/api/ai/indexes/apply?table=products&column=price" \
     -H "Authorization: Bearer <ADMIN_TOKEN>"
```

---

## 7. Audit Trails & Time-Travel Rollback

Every `CREATE`, `UPDATE`, and `DELETE` stores complete JSON diffs and pre-mutation snapshots.

### Restoring Any Modified or Deleted Record
```bash
# 1-Click Rollback to Snapshot
POST /api/d/rollback/42
```
Oxide_CG reads the audit snapshot and seamlessly re-inserts or reverts the record in the database.

---

## 8. Approval Workflows

Fields marked with `.requires_approval()` are intercepted during updates:

1. When a user with the `Editor` role modifies a sensitive field, the change is quarantined in `_oxide_approvals`.
2. The AI Decision Engine evaluates the change and generates an explainable Risk Assessment (`LOW`, `MEDIUM`, `HIGH`, `CRITICAL`).
3. Managers or Admins review the change in the React Admin panel or via API:

```bash
# List pending approvals with AI Risk scores
GET /api/d/approvals

# Approve change and commit to table
POST /api/d/approvals/5/approve

# Reject change
POST /api/d/approvals/5/reject
```

---

## 9. Lifecycle Hooks & Event Bus

You can attach custom logic or webhook notifications to model events:

```rust
use oxide_cg::prelude::*;
use async_trait::async_trait;

struct InventoryAlertHook;

#[async_trait]
impl ModelHook for InventoryAlertHook {
    async fn after_update(&self, model: &str, id: i64, record: &serde_json::Value) -> Result<(), OxideError> {
        if model == "Product" {
            let stock = record.get("stock_quantity").and_then(|v| v.as_i64()).unwrap_or(0);
            if stock < 10 {
                println!("🚨 Low inventory alert for Product #{}: only {} remaining!", id, stock);
            }
        }
        Ok(())
    }
}

// In main.rs:
OxideCGApp::new()
    .register(product_schema)
    .hook(InventoryAlertHook)
    .run()
    .await?;
```

---

## 10. Production Deployment & Kubernetes Setup

### Kubernetes Liveness & Readiness Probes
Configure your `deployment.yaml`:

```yaml
livenessProbe:
  httpGet:
    path: /health/live
    port: 8080
  initialDelaySeconds: 2
  periodSeconds: 5

readinessProbe:
  httpGet:
    path: /health/ready
    port: 8080
  initialDelaySeconds: 2
  periodSeconds: 5
```

### High-Concurrency Linux Kernel Tuning
```bash
sysctl -w fs.file-max=2097152
sysctl -w net.core.somaxconn=65535
sysctl -w net.ipv4.ip_local_port_range="1024 65535"
sysctl -w net.ipv4.tcp_tw_reuse=1
sysctl -p
```
