const { base, page } = document.body.dataset;
const navigation = document.querySelector('nav details');
const desktop = window.matchMedia('(min-width: 901px)');
navigation.open = desktop.matches;
desktop.addEventListener('change', event => { navigation.open = event.matches; });
document.querySelector('#version').addEventListener('change', event => {
  window.location.assign(`${base}${event.target.value}/${page}.html`);
});
const searchInput = document.querySelector('#search');
const results = document.querySelector('#search-results');
let index;
searchInput.addEventListener('input', async () => {
  const query = searchInput.value.trim().toLowerCase();
  results.replaceChildren();
  if (query.length < 2) return;
  try {
    index ??= fetch('search.json').then(response => {
      if (!response.ok) throw new Error('Search unavailable');
      return response.json();
    });
    const entries = await index;
    if (searchInput.value.trim().toLowerCase() !== query) return;
    const matches = entries.filter(entry => query.split(/\s+/).every(word => `${entry.title} ${entry.text}`.toLowerCase().includes(word)));
    for (const entry of matches) {
      const li = document.createElement('li');
      const a = document.createElement('a');
      a.href = entry.url;
      a.textContent = entry.title;
      li.append(a);
      results.append(li);
    }
    if (!matches.length) results.textContent = 'No matching guides.';
  } catch {
    results.textContent = 'Search is unavailable. Browse the guides below.';
    index = undefined;
  }
});
