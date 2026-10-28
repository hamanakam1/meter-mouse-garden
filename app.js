const views = ['garden', 'usage', 'agents', 'providers', 'missions'];
const titles = { garden: 'Your garden is growing', usage: 'What your AI is using', agents: 'Your agents', providers: 'Provider beds', missions: 'Care tasks' };
const toast = document.querySelector('#toast');
let toastTimer;

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
