import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import './App.css';

document.documentElement.classList.add('dark');

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
