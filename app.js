const views = ['garden', 'usage', 'agents', 'providers', 'missions'];
const titles = { garden: 'Your garden is growing', usage: 'What your AI is using', agents: 'Your agents', providers: 'Provider beds', missions: 'Care tasks' };
const toast = document.querySelector('#toast');
let toastTimer;

function setLiveValue(name, value) {
  document.querySelectorAll(`[data-live="${name}"]`).forEach((el) => { el.textContent = value; });
}

async function connectLocalGarden() {
  try {
    const response = await fetch('http://127.0.0.1:8787/api/garden/summary', { signal: AbortSignal.timeout(1200) });
    if (!response.ok) throw new Error(`daemon returned ${response.status}`);
    const summary = await response.json();
    const totalTokens = Number(summary.input_tokens || 0) + Number(summary.output_tokens || 0);
    setLiveValue('requests', Number(summary.requests || 0).toLocaleString());
    setLiveValue('tokens', totalTokens.toLocaleString());
    setLiveValue('observed', `${Number(summary.input_tokens || 0).toLocaleString()} input observed`);
    setLiveValue('providers', Number(summary.providers || 0).toLocaleString());
    setLiveValue('unknown', Number(summary.unknown_values || 0).toLocaleString());
    setLiveValue('visibility', summary.requests ? 'Live' : '0');
    showToast('Meter Mouse connected to your local garden.');
  } catch (_) {
    // Hosted mode remains useful with fixture values when the local daemon is offline.
  }
}

function showToast(message) {
  toast.textContent = message;
  toast.classList.add('show');
  clearTimeout(toastTimer);
  toastTimer = setTimeout(() => toast.classList.remove('show'), 3200);
}

function showView(view) {
  if (!views.includes(view)) return;
  document.querySelectorAll('.view').forEach((el) => el.classList.remove('active-view'));
  document.querySelector(`#view-${view}`).classList.add('active-view');
  document.querySelectorAll('.nav-item').forEach((el) => el.classList.toggle('active', el.dataset.view === view));
  document.querySelector('#page-title').textContent = titles[view];
  window.scrollTo({ top: 0, behavior: 'smooth' });
}

document.addEventListener('click', (event) => {
  const nav = event.target.closest('[data-view]');
  if (nav) showView(nav.dataset.view);
  const target = event.target.closest('[data-view-target]');
  if (target) showView(target.dataset.viewTarget);
  const message = event.target.closest('[data-toast]')?.dataset.toast;
  if (message) showToast(message);
});

connectLocalGarden();
