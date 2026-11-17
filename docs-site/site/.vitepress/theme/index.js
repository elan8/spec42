import DefaultTheme from 'vitepress/theme';
import VersionSwitcher from '@viteplus/versions/components/version-switcher.component.vue';
import { installVersionFallback } from '@viteplus/versions/fallback';
import Layout from './Layout.vue';
import './style.css';

export default {
  extends: DefaultTheme,
  Layout,
  enhanceApp({ app, router, siteData }) {
    app.component('VersionSwitcher', VersionSwitcher);
    installVersionFallback(router, siteData);
  }
};
