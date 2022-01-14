declare interface Window {
  // extend the window
}

/// <reference types="vite-svg-loader" />

// with vite-plugin-md, markdowns can be treat as Vue components
declare module '*.md' {
  import type { ComponentOptions } from 'vue'
  const component: ComponentOptions
  export default component
}
