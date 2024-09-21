// HotkeySettings.jsx

import { useState, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from '@tauri-apps/api/window';
import { emit } from '@tauri-apps/api/event';

const HotkeySettings = () => {
    const [hotkey, setHotkey] = useState<string[]>([]);
    const [displayHotkey, setDisplayHotkey] = useState('Press a key combination');
    const [error, setError] = useState('');
    const hotkeyRef = useRef<HTMLDivElement>(null);

    const handleKeyDown = (event: { preventDefault: () => void; ctrlKey: any; altKey: any; shiftKey: any; metaKey: any; key: string; }) => {
        event.preventDefault();

        // Build the hotkey array
        const keys = [];

        if (event.ctrlKey) keys.push('Ctrl');
        if (event.altKey) keys.push('Alt');
        if (event.shiftKey) keys.push('Shift');
        if (event.metaKey) keys.push('Super'); // For Mac Command key

        const key = event.key.toUpperCase();

        // Ignore modifier-only keys
        if (
            key !== 'CONTROL' &&
            key !== 'ALT' &&
            key !== 'SHIFT' &&
            key !== 'META'
        ) {
            keys.push(key);
        }

        if (keys.length === 0) {
            setError('Please include at least one modifier key (Ctrl, Alt, Shift, Super).');
            return;
        }

        setHotkey(keys);
        setDisplayHotkey(keys.join(' + '));
        setError('');
    };

    const handleSaveHotkey = () => {
        if (hotkey.length === 0) {
            setError('No hotkey selected.');
            return;
        }

        // Invoke the Rust command to update the hotkey
        invoke('update_hotkey', { newHotkey: hotkey })
            .then(() => {
                alert(`Hotkey updated to ${displayHotkey}`);
                setError('');
                emit('hotkey-updated');
                appWindow.close();
            })
            .catch((error) => {
                alert(`Failed to update hotkey: ${error}`);
            });
    };

    useEffect(() => {
        const fetchHotkey = async () => {
            try {
                const fetchedHotkey = await invoke('get_hotkey');
                setHotkey(fetchedHotkey as string[]);
            } catch (error) {
                console.error('Error fetching hotkey:', error);
            }
        };

        fetchHotkey();
    }, []);

    useEffect(() => {
        if (hotkey.length > 0) {
            setDisplayHotkey(hotkey.join(' + '));
        }
    }, [hotkey]);

    useEffect(() => {
        if (hotkeyRef.current) {
            hotkeyRef.current.focus();
        }
    }, []);

    return (
        <div className="h-dvh w-full bg-[#1a1a1a] flex flex-col items-center justify-center select-none p-4">
            <h2 className="text-lg mb-2 font-bold bg-clip-text text-transparent bg-gradient-to-r from-purple-500 via-indigo-600 to-blue-500 animate-gradient-shift">Select a New Hotkey</h2>
            <div
                ref={hotkeyRef}
                onKeyDown={handleKeyDown}
                tabIndex={0}
                className="mb-2 px-1.5 py-1 text-xs font-semibold border rounded-lg bg-gray-600 text-gray-100 border-gray-500 focus:outline-none focus:ring-1 focus:ring-indigo-500 "
            >
                {displayHotkey}
            </div>
            {error && <p className="text-red-500 mb-2">{error}</p>}
            <button
                onClick={handleSaveHotkey}
                className="px-2 py-1 text-xs bg-indigo-600 text-white rounded hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500"
            >
                Save Hotkey
            </button>
        </div>
    );
};

export default HotkeySettings;
