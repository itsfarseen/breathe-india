import "./global.postcss"
import App from './App.svelte'

import TimeAgo from "javascript-time-ago";
import en from "javascript-time-ago/locale/en";
// put here cuz must be called only once.
TimeAgo.addDefaultLocale(en);

const app = new App({
  target: document.getElementById('app')
})

export default app
