<script setup>
import DefaultTheme from 'vitepress/theme';
import VersionNotice from './VersionNotice.vue';
import SourceLink from './SourceLink.vue';

const { Layout } = DefaultTheme;
</script>

<template>
  <Layout>
    <template #doc-before><VersionNotice /></template>
    <template #doc-after><SourceLink /></template>
  </Layout>
</template>
