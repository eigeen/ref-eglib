import { defineConfig } from "vitepress";

export default defineConfig({
  title: "EgLib Document",
  description: "Documentation for EgLib plugin.",

  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    sidebar: [
      {
        text: "Introduction",
        link: "/",
      },
      {
        text: "Types",
        link: "/types",
      },
      {
        text: "Modules",
        base: "/modules/",
        items: [
          { text: "memory", link: "memory" },
          { text: "time", link: "time" },
          { text: "fs", link: "fs" },
          { text: "datetime", link: "datetime" },
        ],
      },
      {
        text: "Objects",
        base: "/objects/",
        items: [
          { text: "LuaPtr", link: "luaptr" },
          { text: "Instant", link: "instant" },
          { text: "Duration", link: "duration" },
          { text: "FsService", link: "FsService" },
          { text: "DateTime", link: "DateTime" },
        ],
      },
    ],
  },
});
