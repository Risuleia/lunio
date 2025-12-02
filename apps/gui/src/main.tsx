import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";

import './index.css'
import AppProvider from "./contexts/AppContext";
import { TabProvider } from "./contexts/TabContext";
import ModalProvider from "./contexts/ModalOverlay";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <AppProvider>
      <TabProvider>
        <ModalProvider>
          <App />
        </ModalProvider>
      </TabProvider>
    </AppProvider>
  </React.StrictMode>,
);
