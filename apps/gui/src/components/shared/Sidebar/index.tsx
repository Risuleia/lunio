import React from 'react';
import { useTabs } from '../../../contexts/TabContext';
import './styles.css'
import { useApp } from '../../../contexts/AppContext';

interface NavItem {
    icon: string,
    label: string,
    location: string
}

const NAVIGATION: NavItem[] = [
  { icon: "home", label: "Home", location: "virtual://home" },
  { icon: "kid_star", label: "Favorites", location: "virtual://favorites" },
  { icon: "schedule", label: "Recent", location: "virtual://recent" },
  { icon: "delete", label: "Trash", location: "virtual://trash" }
];

export default function Sidebar() {
    const { navigate, openTab, getActiveTab } = useTabs()
    const { sidebarEntries } = useApp()
    const active = getActiveTab().location

    function handleClick(e: React.MouseEvent<HTMLButtonElement, MouseEvent>, location: string) {
        if (e.ctrlKey || e.metaKey) openTab(location);
        else navigate(location)
    }
    
    const renderGroup = (items: NavItem[]) =>
        items.map(item => (
            <button
                key={item.location}
                className={`sidebar-link ${active === item.location ? "active" : ""}`}
                onClick={(e) => handleClick(e, item.location)}
            >
                <span className="material-symbols-rounded">{item.icon}</span>
                <span>{item.label}</span>
            </button>
        ));

    const folders = sidebarEntries.filter(e => e.kind === "folder");
    const devices = sidebarEntries.filter(e => e.kind === "drive");

  return (
    <nav id="sidebar">
        <div className="sidebar-section">
            <div className="sidebar-section-container">
                {renderGroup(NAVIGATION)}
            </div>
        </div>
        <div className="sidebar-section">
            <h2 className="sidebar-section-title">This PC</h2>
            <div className="sidebar-section-container">
                {folders.map(item => (
                    <button
                        key={item.path}
                        className={`sidebar-link ${active === item.path ? "active" : ""}`}
                        onClick={(e) => handleClick(e, item.path)}
                    >
                        <span className="material-symbols-rounded">{item.icon}</span>
                        <span>{item.label}</span>
                    </button>
                ))}
            </div>
        </div>
        <div className="sidebar-section">
            <h2 className="sidebar-section-title">Devices</h2>
            <div className="sidebar-section-container">
                {devices.map(item => (
                    <button
                        key={item.path}
                        className={`sidebar-link ${active === item.path ? "active" : ""}`}
                        onClick={(e) => handleClick(e, item.path)}
                    >
                        <span className="material-symbols-rounded">{item.icon}</span>
                        <span>{item.label}</span>
                    </button>
                ))}
            </div>
        </div>
    </nav>
  )
}
