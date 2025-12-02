import React, { createContext, useContext, useEffect, useState } from "react";
import { connect, getSidebarEntries, SystemEntry } from "../services/daemon";

type Platform = "windows" | "macos" | "linux";

interface AppState {
    platform: Platform;
    vibrancy: boolean;
    ready: boolean;
    error: string | null;
}

type AppContextType = AppState & {
    setVibrancy: (v: boolean) => void;
    setReady: (v: boolean) => void;
    sidebarEntries: SystemEntry[]
}

const AppContext = createContext<AppContextType | null>(null)

function detectPlatform(): Platform {
    const p = navigator.platform.toLowerCase()
    if (p.includes("mac")) return "macos";
    if (p.includes("wind")) return "windows";
    return "linux"
}

export default function AppProvider({ children }: { children: React.ReactNode }) {
    const platform = detectPlatform();
    const [vibrancy, setVibrancy] = useState<boolean>(platform !== "linux")
    const [ready, setReady] = useState(false)
    const [error, setError] = useState<string | null>(null)
    const [sidebarEntries, setSidebarEntries] = useState<SystemEntry[]>([])

    useEffect(() => {
        if (platform === "linux") setVibrancy(false);
    }, [platform]);

    useEffect(() => {
        if (!vibrancy) document.documentElement.classList.add("linux")
    }, [vibrancy])

    useEffect(() => {
        async function boot(retries = 5) {
            for (let i = 0; i < retries; i++) {
                try {
                    await connect()
                    setReady(true)

                    const entries = await getSidebarEntries()
                    setSidebarEntries(entries);

                    return
                } catch {
                    await new Promise(r => setTimeout(r, 1000));
                }
            }

            setError("Backend failed to start");
        }

        boot()
    }, [])

    const value: AppContextType = {
        platform,
        vibrancy,
        ready,
        error,
        sidebarEntries,
        setVibrancy,
        setReady
    }

    return <AppContext.Provider value={value}>{children}</AppContext.Provider>
}

export function useApp() {
    const ctx = useContext(AppContext)
    if (!ctx) throw new Error("useApp must be used inside AppProvider")
    
    return ctx
}