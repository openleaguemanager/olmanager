import React, { useEffect, useState } from "react";
import ReactDOM from "react-dom/client";
import { ThemeProvider } from "./context/ThemeContext";
import "./i18n";
import App from "./App";
import AppV2 from "./ui-v2/AppV2";
import { useUIVersion } from "./ui-v2/uiVersion";
import { getApiClient } from "./api/client";

// Disable the native browser context menu in the Tauri app.
document.addEventListener("contextmenu", (event) => {
  event.preventDefault();
});

function Root() {
  const version = useUIVersion();
  return version === "v2" ? <AppV2 /> : <App />;
}

function Boot() {
  const [ready, setReady] = useState(false);
  useEffect(() => {
    getApiClient().then(() => setReady(true));
  }, []);
  if (!ready) return null;
  return (
    <ThemeProvider>
      <Root />
    </ThemeProvider>
  );
}

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <Boot />
  </React.StrictMode>,
);
