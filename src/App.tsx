import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { getVersion } from '@tauri-apps/api/app';
import ZapLinkLogo from "./assets/zaplink.svg";
import "./App.css";

const AppVersion = () => {
  const [version, setVersion] = useState('');

  useEffect(() => {
    async function fetchVersion() {
      const appVersion = await getVersion();
      setVersion(appVersion);
    }

    fetchVersion();
  }, []);

  useEffect(() => {
    setTimeout(() => {
      invoke('close_splashscreen');
    }, 1000);
  }, []);

  return (
    <div className="h-dvh w-dvw bg-[#1a1a1a] flex flex-col gap-1 items-center justify-center select-none">
      <div className="flex gap-1 items-center">
        <img src={ZapLinkLogo} className="h-6 w-6" alt="React Logo" />
        <h1 className="text-xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-purple-500 to-blue-500">
          ZapLink v.{version}
        </h1>
      </div>
      <p className="text-xs font-extrabold bg-clip-text text-transparent bg-gradient-to-r from-purple-500 to-blue-500">
        ALT + V
      </p>
      <a href="https://github.com/infinitel8p/zaplink" className="absolute bottom-0.5 text-xs text-gray-700 hover:underline">Visit Repository</a>
    </div>
  );
};

export default AppVersion;
