import { useState, useEffect } from "react";
import ZapLinkLogo from "./assets/zaplink.svg";
// import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { getVersion } from '@tauri-apps/api/app';

const AppVersion = () => {
  const [version, setVersion] = useState('');

  useEffect(() => {
    async function fetchVersion() {
      const appVersion = await getVersion();
      setVersion(appVersion);
    }

    fetchVersion();
  }, []);

  return (
    <div className="h-dvh w-dvw bg-[#202020] flex items-center justify-center">
      <img src={ZapLinkLogo} className="h-16 w-16 fill-gradient-to-r from-purple-500 to-blue-500" alt="React Logo" />
      <p className="text-xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-purple-500 to-blue-500">
        ZapLink v.{version}
      </p>
    </div>
  );
};

export default AppVersion;
