import { useEffect, useState } from "react";
import { ExplorerItem } from "../constants/ExplorerItem";
import { listDir } from "../services/daemon";
import { adaptEntry } from "../lib/adapt";
import { TabState } from "../contexts/TabContext";

interface UseEntriesType {
    entries: ExplorerItem[],
    loading: boolean,
    refresh: () => void
}

export default function useEntries(tab: TabState): UseEntriesType {
    const [entries, setEntries] = useState<ExplorerItem[]>([])
    const [loading, setLoading] = useState(false)

    async function getEntries() {
        const location = tab.location
        if (location.startsWith("virtual://")) return setEntries([])
        
        console.log("[Explorer] requesting listDir:", location);
        
        setLoading(true)
        setEntries([])

        try {
            const data = await listDir(location)
            setEntries((data || []).map(adaptEntry))    
        } catch (e) {
            console.error(e)
        } finally {
            setLoading(false)
        }
    }

    useEffect(() => {
        getEntries()
    }, [tab.location])

    function refresh() {
        getEntries()
    }

    return { entries, loading, refresh }
}