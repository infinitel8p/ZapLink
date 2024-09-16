import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { getVersion } from '@tauri-apps/api/app';
import ZapLinkLogo from "./assets/zaplink.svg";
import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/api/notification';
import { WebviewWindow } from '@tauri-apps/api/window';
import { listen } from '@tauri-apps/api/event';
import "./App.css";

const AppVersion = () => {
  const [version, setVersion] = useState('');
  const [latestVersion, setLatestVersion] = useState('');
  const [hotkey, setHotkey] = useState<string[]>([]);

  const openHotkeySettings = () => {
    // Check if the window already exists to prevent duplicates
    let hotkeyWindow = WebviewWindow.getByLabel('hotkey-settings');

    if (!hotkeyWindow) {
      // Create a new window
      hotkeyWindow = new WebviewWindow('hotkey-settings', {
        url: `${window.location.origin}/#/hotkey-settings`,
        title: 'Change Hotkey',
        width: 250,
        height: 150,
        resizable: false,
        visible: true,
        center: true,
      });

      // Optional: Handle events or errors
      hotkeyWindow.once('tauri://error', (e) => {
        console.error('Failed to create hotkey settings window', e);
      });
    } else {
      // If the window already exists, just show it
      hotkeyWindow.show();
      hotkeyWindow.setFocus();
    }
  };

  const fetchHotkey = async () => {
    try {
      const fetchedHotkey = await invoke('get_hotkey');
      setHotkey(fetchedHotkey as string[]);
    } catch (error) {
      console.error('Error fetching hotkey:', error);
    }
  };

  useEffect(() => {
    fetchHotkey();

    // Listen for the hotkey-updated event
    const unlisten = listen('hotkey-updated', () => {
      fetchHotkey();
    });

    return () => {
      // Clean up the event listener when the component is unmounted
      unlisten.then((f) => f());
    };
  }, []);

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

  const isUpdateAvailable = version < latestVersion;

  const handleNotification = async () => {
    let permissionGranted = await isPermissionGranted();
    if (!permissionGranted) {
      const permission = await requestPermission();
      permissionGranted = permission === 'granted';
    }
    if (permissionGranted) {
      sendNotification({ title: 'ZapLink', body: 'Update Available', icon: ZapLinkLogo });
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
        {hotkey.map((key, index) => (
          <span key={index}>
            <kbd className="px-1.5 py-1 text-xs font-semibold border rounded-lg bg-gray-600 text-gray-100 border-gray-500">{key}</kbd>
            {index < hotkey.length - 1 && ' + '}
          </span>
        ))}
      </p>
      {isUpdateAvailable && (
        <a href="https://github.com/infinitel8p/zaplink/releases/latest" className="bg-indigo-600 px-4 py-0.5 text-gray-100 absolute top-0 w-full text-center text-xs font-medium inline-block underline" target="_blank">
          Update Available: v.{latestVersion}
        </a>
      )}

      <div className="absolute bottom-0.5 text-xs text-gray-700 flex items-end gap-2">
        <button onClick={openHotkeySettings} className=" hover:underline">
          Change Hotkey
        </button>
        <a href="https://github.com/infinitel8p/zaplink" target="_blank" className=" hover:underline">Visit Repository</a>
      </div>
    </div>
  );
};

export default AppVersion;
