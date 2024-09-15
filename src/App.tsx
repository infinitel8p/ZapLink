import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { getVersion } from '@tauri-apps/api/app';
import ZapLinkLogo from "./assets/zaplink.svg";
import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/api/notification';
import "./App.css";

const AppVersion = () => {
  const [version, setVersion] = useState('');
  const [latestVersion, setLatestVersion] = useState('');

  useEffect(() => {
    const fetchVersion = async () => {
      const appVersion = await getVersion();
      setVersion(appVersion);

      const response = await fetch('https://api.github.com/repos/infinitel8p/zaplink/releases/latest');
      const data = await response.json();
      setLatestVersion(data.tag_name);
    };

    fetchVersion();
  }, []);

  const isUpdateAvailable = version == latestVersion;

  const handleNotification = async () => {
    let permissionGranted = await isPermissionGranted();
    if (!permissionGranted) {
      const permission = await requestPermission();
      permissionGranted = permission === 'granted';
    }
    if (permissionGranted) {
      sendNotification({ title: 'ZapLink', body: 'Update Available' });
    }
  };

  useEffect(() => {
    setTimeout(() => {
      invoke('close_splashscreen').then(() => {
        if (isUpdateAvailable) {
          invoke('unhide_window');
          handleNotification();
        }
      });
    }, 1000);
  }, [isUpdateAvailable]);

  return (
    <div className="h-dvh w-dvw bg-[#1a1a1a] flex flex-col gap-1 items-center justify-center select-none">
      <div className="flex gap-1 items-center">
        <img src={ZapLinkLogo} className="h-6 w-6" alt="React Logo" />
        <h1 className="text-2xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-purple-500 via-indigo-600 to-blue-500">
          ZapLink v.{version}
        </h1>
      </div>
      <p className="text-gray-400">
        <kbd className="px-1.5 py-1 text-xs font-semibold border rounded-lg bg-gray-600 text-gray-100 border-gray-500">Alt</kbd> + <kbd className="px-1.5 py-1 text-xs font-semibold border rounded-lg bg-gray-600 text-gray-100 border-gray-500">V</kbd>
      </p>
      {isUpdateAvailable && (
        <a href="https://github.com/infinitel8p/zaplink/releases/latest" className="bg-indigo-600 px-4 py-0.5 text-gray-100 absolute top-0 w-full text-center text-xs font-medium inline-block underline" target="_blank">
          Update Available: v.{latestVersion}
        </a>
      )}
      <a href="https://github.com/infinitel8p/zaplink" className="absolute bottom-0.5 text-xs text-gray-700 hover:underline" target="_blank">Visit Repository</a>
    </div>
  );
};

export default AppVersion;
