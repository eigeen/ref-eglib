import { defineConfig } from "vitepress";

export default defineConfig({
  title: "EgLib 文档",
  description: "EgLib 插件文档",

  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    sidebar: [
      {
        text: "介绍",
        link: "/zh/",
      },
      {
        text: "类型别名",
        link: "/zh/types",
      },
      {
        text: "模块",
        base: "/zh/modules/",
        items: [
          { text: "memory", link: "memory" },
          { text: "time", link: "time" },
          { text: "fs", link: "fs" },
        ],
      },
      {
        text: "对象",
        base: "/zh/objects/",
        items: [
          { text: "LuaPtr", link: "luaptr" },
          { text: "Instant", link: "instant" },
          { text: "Duration", link: "duration" },
          { text: "FsService", link: "FsService" },
        ],
      },
    ],
    outlineTitle: "页面内容",
    docFooter: {
      prev: "上一篇",
      next: "下一篇",
    },
  },
});
