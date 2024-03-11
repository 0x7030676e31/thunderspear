// vite.config.ts
import { defineConfig } from "file:///home/manjaro/@thunderspear/node_modules/vite/dist/node/index.js";
import solid from "file:///home/manjaro/@thunderspear/node_modules/vite-plugin-solid/dist/esm/index.mjs";
var vite_config_default = defineConfig(async () => ({
  plugins: [solid()],
  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"]
    }
  }
}));
export {
  vite_config_default as default
};
//# sourceMappingURL=data:application/json;base64,ewogICJ2ZXJzaW9uIjogMywKICAic291cmNlcyI6IFsidml0ZS5jb25maWcudHMiXSwKICAic291cmNlc0NvbnRlbnQiOiBbImNvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9kaXJuYW1lID0gXCIvaG9tZS9tYW5qYXJvL0B0aHVuZGVyc3BlYXJcIjtjb25zdCBfX3ZpdGVfaW5qZWN0ZWRfb3JpZ2luYWxfZmlsZW5hbWUgPSBcIi9ob21lL21hbmphcm8vQHRodW5kZXJzcGVhci92aXRlLmNvbmZpZy50c1wiO2NvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9pbXBvcnRfbWV0YV91cmwgPSBcImZpbGU6Ly8vaG9tZS9tYW5qYXJvL0B0aHVuZGVyc3BlYXIvdml0ZS5jb25maWcudHNcIjtpbXBvcnQgeyBkZWZpbmVDb25maWcgfSBmcm9tIFwidml0ZVwiO1xuaW1wb3J0IHNvbGlkIGZyb20gXCJ2aXRlLXBsdWdpbi1zb2xpZFwiO1xuXG4vLyBodHRwczovL3ZpdGVqcy5kZXYvY29uZmlnL1xuZXhwb3J0IGRlZmF1bHQgZGVmaW5lQ29uZmlnKGFzeW5jICgpID0+ICh7XG4gIHBsdWdpbnM6IFtzb2xpZCgpXSxcblxuICAvLyBWaXRlIG9wdGlvbnMgdGFpbG9yZWQgZm9yIFRhdXJpIGRldmVsb3BtZW50IGFuZCBvbmx5IGFwcGxpZWQgaW4gYHRhdXJpIGRldmAgb3IgYHRhdXJpIGJ1aWxkYFxuICAvL1xuICAvLyAxLiBwcmV2ZW50IHZpdGUgZnJvbSBvYnNjdXJpbmcgcnVzdCBlcnJvcnNcbiAgY2xlYXJTY3JlZW46IGZhbHNlLFxuICAvLyAyLiB0YXVyaSBleHBlY3RzIGEgZml4ZWQgcG9ydCwgZmFpbCBpZiB0aGF0IHBvcnQgaXMgbm90IGF2YWlsYWJsZVxuICBzZXJ2ZXI6IHtcbiAgICBwb3J0OiAxNDIwLFxuICAgIHN0cmljdFBvcnQ6IHRydWUsXG4gICAgd2F0Y2g6IHtcbiAgICAgIC8vIDMuIHRlbGwgdml0ZSB0byBpZ25vcmUgd2F0Y2hpbmcgYHNyYy10YXVyaWBcbiAgICAgIGlnbm9yZWQ6IFtcIioqL3NyYy10YXVyaS8qKlwiXSxcbiAgICB9LFxuICB9LFxufSkpO1xuIl0sCiAgIm1hcHBpbmdzIjogIjtBQUFtUSxTQUFTLG9CQUFvQjtBQUNoUyxPQUFPLFdBQVc7QUFHbEIsSUFBTyxzQkFBUSxhQUFhLGFBQWE7QUFBQSxFQUN2QyxTQUFTLENBQUMsTUFBTSxDQUFDO0FBQUE7QUFBQTtBQUFBO0FBQUEsRUFLakIsYUFBYTtBQUFBO0FBQUEsRUFFYixRQUFRO0FBQUEsSUFDTixNQUFNO0FBQUEsSUFDTixZQUFZO0FBQUEsSUFDWixPQUFPO0FBQUE7QUFBQSxNQUVMLFNBQVMsQ0FBQyxpQkFBaUI7QUFBQSxJQUM3QjtBQUFBLEVBQ0Y7QUFDRixFQUFFOyIsCiAgIm5hbWVzIjogW10KfQo=
