import React from "react";
import ReactDOM from "react-dom/client";
import { ThemeProvider } from "./context/ThemeContext";
import "./i18n";
import App from "./App";
import AppV2 from "./ui-v2/AppV2";
import { useUIVersion } from "./ui-v2/uiVersion";
import { AuthGate, AuthProvider } from "./web/auth";
import { getApiClient } from "./api/client";

// Disable the native browser context menu in the Tauri app.
// Custom context menus (e.g. <ContextMenu>) handle their own onContextMenu
// events and call stopPropagation(), so they continue to work.
document.addEventListener("contextmenu", (event) => {
  event.preventDefault();
});

function Root() {
  const version = useUIVersion();
  return version === "v2" ? <AppV2 /> : <App />;
}

// Initialize the API client before anything tries to use it
getApiClient();

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <ThemeProvider>
      {import.meta.env.MODE === "web" ? (
        <AuthProvider>
          <AuthGate>
            <Root />
          </AuthGate>
        </AuthProvider>
      ) : (
        <Root />
      )}
    </ThemeProvider>
  </React.StrictMode>,
);
