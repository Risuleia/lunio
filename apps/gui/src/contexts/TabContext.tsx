import React, { createContext, useContext, useState } from "react";
import { GroupMode } from "../lib/grouping/types";
import { SortMode, SortOrder } from "../lib/sorting/types";

export type ViewMode = "grid" | "list" | "masonry";

export interface TabState {
    id: string,
    location: string,
    history: string[],
    historyIndex: number,
    viewMode: ViewMode,
    groupMode: GroupMode,
    sortMode: SortMode,
    sortOrder: SortOrder,
    scrollTop: number,
    selection: string[]
}

interface TabContextType {
    tabs: TabState[],
    activeTabId: string,

    openTab: (location: string) => void,
    closeTab: (id: string) => void,
    navigate: (location: string, newTab?: boolean) => void,
    canGoBack: () => boolean,
    canGoForward: () => boolean,
    goBack: () => void,
    goForward: () => void,
    setViewMode: (mode: ViewMode) => void,
    setGroupMode: (mode: GroupMode) => void,
    setSortMode: (mode: SortMode) => void,
    setSortOrder: (order: SortOrder) => void,
    setActiveTab: (id: string) => void,
    getActiveTab(): TabState
    updateScroll: (scrollTop: number) => void,
}

const TabContext = createContext<TabContextType | null>(null)

function createTab(location: string): TabState {
    return {
        id: crypto.randomUUID(),
        location,
        history: [location],
        historyIndex: 0,
        viewMode: "grid",
        groupMode: "none",
        sortMode: "name",
        sortOrder: "asc",
        scrollTop: 0,
        selection: []
    }
}

export function TabProvider({ children }: { children: React.ReactNode }) {
    const [tabs, setTabs] = useState<TabState[]>([createTab("virtual://home")])
    const [activeTabId, setActiveTabId] = useState<string>(tabs[0].id)

    function getActiveTab(): TabState {
        const tab = tabs.find(t => t.id == activeTabId)
        return tab ?? tabs[0]
    }

    function setActiveTab(id: string) {
        setActiveTabId(id)
    }

    function openTab(location: string) {
        const tab = createTab(location)
        setTabs(t => [...t, tab])
        setActiveTabId(tab.id)
    }

    function closeTab(id: string) {
        setTabs(t => {
            if (t.length == 1) return t;

            const idx = t.findIndex(tab => tab.id == id)
            const filtered = t.filter(tab => tab.id != id)

            if (id == activeTabId) {
                const next = filtered[Math.max(0, idx - 1)];
                setActiveTabId(next.id)
            }

            return filtered
        })
    }

    function navigate(location: string, newTab = false) {
        if (newTab) return openTab(location)

        setTabs(t =>
            t.map(tab => {
                if (tab.id !== activeTabId) return tab

                const nextHistory = tab.history.slice(0, tab.historyIndex + 1)

                return {
                    ...tab,
                    location,
                    history: [...nextHistory, location],
                    historyIndex: nextHistory.length,
                    scrollTop: 0,
                    selection: []
                };
            })
        );
    }

    function canGoBack(): boolean {
        return getActiveTab().historyIndex > 0
    }

    function canGoForward(): boolean {
        return getActiveTab().historyIndex < getActiveTab().history.length - 1
    }

    function goBack() {
        setTabs(tabs =>
            tabs.map(tab => {
                if (tab.id !== activeTabId) return tab
                if (tab.historyIndex === 0) return tab

                const newIndex = tab.historyIndex - 1

                return {
                    ...tab,
                    historyIndex: newIndex,
                    location: tab.history[newIndex]
                }
            })
        )
    }

    function goForward() {
        setTabs(tabs =>
            tabs.map(tab => {
                if (tab.id !== activeTabId) return tab
                if (tab.historyIndex >= tab.history.length - 1) return tab

                const newIndex = tab.historyIndex + 1

                return {
                    ...tab,
                    historyIndex: newIndex,
                    location: tab.history[newIndex]
                }
            })
        )
    }

    function updateScroll(scrollTop: number) {
        setTabs(t =>
            t.map(tab =>
                tab.id === activeTabId
                    ? { ...tab, scrollTop }
                    : tab
            )
        )
    }

    function setViewMode(mode: ViewMode) {
        setTabs(t => 
            t.map(tab => 
                tab.id == activeTabId ? { ...tab, viewMode: mode } : tab
            )
        )
    }
    function setGroupMode(mode: GroupMode) {
        setTabs(t => 
            t.map(tab => 
                tab.id == activeTabId ? { ...tab, groupMode: mode } : tab
            )
        )
    }
    function setSortMode(mode: SortMode) {
        setTabs(t => 
            t.map(tab => 
                tab.id == activeTabId ? { ...tab, sortMode: mode } : tab
            )
        )
    }
    function setSortOrder(order: SortOrder) {
        setTabs(t => 
            t.map(tab => 
                tab.id == activeTabId ? { ...tab, sortOrder: order } : tab
            )
        )
    }

    const value: TabContextType = {
        tabs,
        activeTabId,
        openTab,
        closeTab,
        navigate,
        canGoBack,
        canGoForward,
        goBack,
        goForward,
        setViewMode,
        setGroupMode,
        setSortMode,
        setSortOrder,
        getActiveTab,
        setActiveTab,
        updateScroll
    }

    return <TabContext.Provider value={value}>{children}</TabContext.Provider>
}

export function useTabs() {
    const ctx = useContext(TabContext)
    if (!ctx) throw new Error("useTab must be used inside TabProvider")

    return ctx
}