import React from "react";
import ReactDOM from "react-dom/client";
import { HashRouter as Router, Routes, Route } from 'react-router-dom';
import App from "./App";
import HotkeySettings from './HotkeySettings';
import "./App.css";

document.documentElement.classList.add('dark');

const root = ReactDOM.createRoot(document.getElementById('root') as HTMLElement);
root.render(
  <React.StrictMode>
    <Router>
      <Routes>
        <Route path="/" element={<App />} />
        <Route path="/hotkey-settings" element={<HotkeySettings />} />
      </Routes>
    </Router >
  </React.StrictMode>
);