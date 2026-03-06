use std::sync::Arc;

use axum::{
    Router,
    extract::{Path, State},
    response::Html,
    routing::get,
    Json,
};

use crate::core::{Change, Part, PlmError};
use crate::infra::JsonRepo;
use crate::core::Repo;

type AppState = Arc<JsonRepo>;

pub async fn serve(repo: JsonRepo, port: u16) -> Result<(), PlmError> {
    let state = Arc::new(repo);

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/api/parts", get(list_parts_handler))
        .route("/api/parts/{number}/history", get(part_history_handler))
        .with_state(state);

    let addr = format!("127.0.0.1:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .map_err(|e| PlmError::Storage(e.to_string()))?;

    println!("Serving mini-PLM at http://{}", addr);

    axum::serve(listener, app)
        .await
        .map_err(|e| PlmError::Storage(e.to_string()))
}

async fn index_handler() -> Html<&'static str> {
    Html(UI_HTML)
}

async fn list_parts_handler(
    State(repo): State<AppState>,
) -> Result<Json<Vec<Part>>, String> {
    repo.load()
        .map(|snap| Json(snap.parts))
        .map_err(|e| e.to_string())
}

async fn part_history_handler(
    State(repo): State<AppState>,
    Path(number): Path<String>,
) -> Result<Json<Vec<Change>>, String> {
    repo.load()
        .map(|snap| {
            Json(
                snap.changes
                    .into_iter()
                    .filter(|c| c.part_number == number)
                    .collect(),
            )
        })
        .map_err(|e| e.to_string())
}

static UI_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>mini-PLM</title>
  <style>
    *, *::before, *::after { box-sizing: border-box; margin: 0; padding: 0; }

    body {
      font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
      background: #f0f2f5;
      color: #111827;
      min-height: 100vh;
    }

    header {
      background: #0f172a;
      color: #f8fafc;
      padding: 1.25rem 2rem;
      display: flex;
      align-items: baseline;
      gap: 0.75rem;
    }
    header h1 { font-size: 1.25rem; font-weight: 700; letter-spacing: -0.01em; }
    header p  { font-size: 0.82rem; color: #94a3b8; }

    main { max-width: 960px; margin: 2rem auto; padding: 0 1.5rem; }

    .card {
      background: #fff;
      border-radius: 14px;
      box-shadow: 0 1px 3px rgba(0,0,0,0.07), 0 4px 16px rgba(0,0,0,0.04);
      overflow: hidden;
    }

    .card-header {
      display: flex;
      align-items: center;
      justify-content: space-between;
      padding: 1.1rem 1.5rem;
      border-bottom: 1px solid #f1f5f9;
    }
    .card-header-left { display: flex; align-items: center; gap: 0.65rem; }
    .card-header h2   { font-size: 0.95rem; font-weight: 600; }

    .pill {
      background: #eff6ff;
      color: #2563eb;
      border-radius: 999px;
      padding: 0.15rem 0.55rem;
      font-size: 0.72rem;
      font-weight: 700;
    }

    .btn-refresh {
      background: none;
      border: 1px solid #e2e8f0;
      border-radius: 8px;
      padding: 0.35rem 0.85rem;
      font-size: 0.78rem;
      color: #64748b;
      cursor: pointer;
      transition: background 0.12s, color 0.12s;
    }
    .btn-refresh:hover { background: #f8fafc; color: #0f172a; }

    table { width: 100%; border-collapse: collapse; }

    thead th {
      background: #f8fafc;
      padding: 0.65rem 1.25rem;
      text-align: left;
      font-size: 0.7rem;
      font-weight: 600;
      text-transform: uppercase;
      letter-spacing: 0.06em;
      color: #94a3b8;
      border-bottom: 1px solid #f1f5f9;
    }

    tbody tr.part-row {
      cursor: pointer;
      transition: background 0.1s;
    }
    tbody tr.part-row:hover { background: #f8fafc; }

    tbody td {
      padding: 0.9rem 1.25rem;
      border-bottom: 1px solid #f1f5f9;
      font-size: 0.875rem;
      vertical-align: middle;
    }

    .td-chevron { width: 2rem; color: #cbd5e1; }

    .chevron {
      display: inline-block;
      transition: transform 0.2s ease;
      font-style: normal;
      font-size: 1rem;
      line-height: 1;
      user-select: none;
    }
    .chevron.open { transform: rotate(90deg); color: #3b82f6; }

    .part-num {
      font-family: 'SF Mono', 'Fira Code', monospace;
      font-size: 0.88rem;
      font-weight: 600;
    }

    .badge {
      display: inline-flex;
      align-items: center;
      gap: 0.3rem;
      padding: 0.2rem 0.65rem;
      border-radius: 999px;
      font-size: 0.72rem;
      font-weight: 600;
    }
    .badge::before { content: ''; width: 6px; height: 6px; border-radius: 50%; background: currentColor; opacity: 0.7; }
    .badge-Design     { background: #eff6ff; color: #1d4ed8; }
    .badge-Prototype  { background: #fff7ed; color: #c2410c; }
    .badge-Production { background: #f0fdf4; color: #15803d; }
    .badge-Obsolete   { background: #f8fafc; color: #64748b; }

    tr.expand-row td {
      padding: 0;
      background: #fafbfd;
    }

    .expand-inner {
      overflow: hidden;
      max-height: 0;
      transition: max-height 0.25s ease;
    }
    .expand-inner.open { max-height: 800px; }

    .history-wrap { padding: 0.75rem 1.25rem 0.75rem 3.25rem; }

    .history-label {
      font-size: 0.68rem;
      font-weight: 700;
      text-transform: uppercase;
      letter-spacing: 0.07em;
      color: #94a3b8;
      margin-bottom: 0.5rem;
    }

    .history-table { width: 100%; border-collapse: collapse; }
    .history-table th {
      text-align: left;
      font-size: 0.68rem;
      font-weight: 600;
      text-transform: uppercase;
      letter-spacing: 0.05em;
      color: #94a3b8;
      padding: 0.4rem 0.75rem;
      background: #f1f5f9;
      border-radius: 4px 4px 0 0;
    }
    .history-table td {
      padding: 0.55rem 0.75rem;
      font-size: 0.8rem;
      color: #374151;
      background: #fff;
      border-top: 1px solid #f1f5f9;
    }
    .history-table tr:last-child td { border-bottom: 1px solid #f1f5f9; }
    .history-table .eco { font-family: monospace; font-weight: 600; font-size: 0.82rem; }
    .history-table .reason { color: #6b7280; font-style: italic; }
    .history-table .date { color: #9ca3af; font-size: 0.75rem; white-space: nowrap; }

    .no-history {
      padding: 0.75rem 0;
      font-size: 0.8rem;
      color: #94a3b8;
      font-style: italic;
    }

    .empty-state {
      text-align: center;
      padding: 3.5rem 1rem;
      color: #94a3b8;
    }
    .empty-state p { font-size: 0.875rem; margin-top: 0.4rem; }

    .error-state {
      text-align: center;
      padding: 2rem;
      color: #ef4444;
      font-size: 0.875rem;
    }
  </style>
</head>
<body>

<header>
  <h1>mini-PLM</h1>
  <p>Parts &amp; Lifecycle Manager</p>
</header>

<main>
  <div class="card">
    <div class="card-header">
      <div class="card-header-left">
        <h2>Parts</h2>
        <span class="pill" id="count">—</span>
      </div>
      <button class="btn-refresh" onclick="load()">↻ Refresh</button>
    </div>
    <table>
      <thead>
        <tr>
          <th class="td-chevron"></th>
          <th>Part number</th>
          <th>Lifecycle</th>
        </tr>
      </thead>
      <tbody id="tbody"></tbody>
    </table>
  </div>
</main>

<script>
  const histCache = {};

  function badge(lc) {
    return `<span class="badge badge-${lc}">${lc}</span>`;
  }

  async function toggleRow(number, idx) {
    const inner = document.getElementById(`expand-${idx}`);
    const chev  = document.getElementById(`chev-${idx}`);
    const opening = !inner.classList.contains('open');

    inner.classList.toggle('open', opening);
    chev.classList.toggle('open', opening);

    if (opening && histCache[number] === undefined) {
      document.getElementById(`hist-${idx}`).innerHTML =
        '<div class="no-history">Loading…</div>';
      try {
        const res = await fetch(`/api/parts/${encodeURIComponent(number)}/history`);
        histCache[number] = await res.json();
      } catch {
        histCache[number] = null;
      }
      renderHist(number, idx);
    }
  }

  function renderHist(number, idx) {
    const el    = document.getElementById(`hist-${idx}`);
    const items = histCache[number];

    if (!items) {
      el.innerHTML = '<div class="no-history">Failed to load history.</div>';
      return;
    }
    if (items.length === 0) {
      el.innerHTML = '<div class="no-history">No changes recorded yet.</div>';
      return;
    }

    el.innerHTML = `
      <table class="history-table">
        <thead>
          <tr>
            <th>ECO</th>
            <th>From</th>
            <th>To</th>
            <th>Reason</th>
            <th>Date (UTC)</th>
          </tr>
        </thead>
        <tbody>
          ${items.map(c => `
          <tr>
            <td class="eco">${c.eco}</td>
            <td>${badge(c.from)}</td>
            <td>${badge(c.to)}</td>
            <td class="reason">${c.reason}</td>
            <td class="date">${new Date(c.at_utc).toLocaleString()}</td>
          </tr>`).join('')}
        </tbody>
      </table>`;
  }

  async function load() {
    const tbody = document.getElementById('tbody');
    const count = document.getElementById('count');

    tbody.innerHTML = '';
    count.textContent = '—';

    let parts;
    try {
      const res = await fetch('/api/parts');
      parts = await res.json();
    } catch (e) {
      tbody.innerHTML = `<tr><td colspan="3" class="error-state">Failed to load parts: ${e.message}</td></tr>`;
      return;
    }

    count.textContent = parts.length;

    if (parts.length === 0) {
      tbody.innerHTML = `
        <tr><td colspan="3">
          <div class="empty-state">
            <strong>No parts yet</strong>
            <p>Create one via the CLI: <code>mini-plm create P-1001</code></p>
          </div>
        </td></tr>`;
      return;
    }

    tbody.innerHTML = parts.map((p, i) => `
      <tr class="part-row" onclick="toggleRow('${p.number}', ${i})">
        <td class="td-chevron"><i class="chevron" id="chev-${i}">›</i></td>
        <td class="part-num">${p.number}</td>
        <td>${badge(p.lifecycle)}</td>
      </tr>
      <tr class="expand-row">
        <td colspan="3">
          <div class="expand-inner" id="expand-${i}">
            <div class="history-wrap">
              <div class="history-label">Change history</div>
              <div id="hist-${i}"></div>
            </div>
          </div>
        </td>
      </tr>`).join('');
  }

  load();
</script>
</body>
</html>
"#;
