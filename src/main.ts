import "./styles.scss";
import App from "./App.svelte";

const app = new App({
  // @ts-ignore idk why this errors
  target: document.getElementById("app"),
});

export default app;
