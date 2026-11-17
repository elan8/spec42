<script setup>
import { useData } from 'vitepress';

// Pages generated from a tagged source name it, and the revision it came from, in front matter.
const { frontmatter, theme } = useData();
</script>

<template>
  <div v-if="frontmatter.source && frontmatter.ref" class="source-link">
    <a :href="`https://github.com/${theme.elan8Docs.repo}/blob/${frontmatter.ref}/${frontmatter.source}`">
      View source for this version
    </a>
  </div>
</template>
