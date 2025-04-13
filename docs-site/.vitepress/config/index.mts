import { defineConfig } from 'vitepress'
import shared from './shared'
import zh from './zh'
import en from './en'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  ...shared,

  locales: {
    root: {
      label: 'English',
      lang: 'en',
      ...en,
    },
    zh: {
      label: '简体中文',
      lang: 'zh', // 可选，将作为 `lang` 属性添加到 `html` 标签中
      link: '/zh/', // 默认 /zh/ -- 显示在导航栏翻译菜单上，可以是外部的
      ...zh,
    }
  }
})
