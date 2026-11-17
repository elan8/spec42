<script setup>
import { computed } from 'vue';
import { useData, useRoute, withBase } from 'vitepress';

const { theme, site } = useData();
const route = useRoute();

// `/<base>/<version>/<page>`: the plugin serves every version under its own folder.
const location = computed(() => {
  const base = site.value.base;
  const rest = route.path.startsWith(base) ? route.path.slice(base.length) : route.path.replace(/^\//, '');
  const [version, ...page] = rest.split('/');
  return { version, page: page.join('/') };
});

const notice = computed(() => {
  const { latest, development } = theme.value.elan8Docs ?? {};
  const { version, page } = location.value;
  if (!latest || version === latest) return null;
  if (version === development) return { development: true, text: 'Development documentation describes unreleased changes.' };
  if (!/^v\d/.test(version)) return null;
  return { development: false, text: `You are reading ${version}.`, link: withBase(`/${latest}/${page}`), linkText: 'Read the latest stable guide' };
});
</script>

<template>
  <div v-if="notice" class="version-notice" :class="{ development: notice.development }">
    {{ notice.text }}
    <a v-if="notice.link" :href="notice.link">{{ notice.linkText }}</a>
  </div>
</template>
