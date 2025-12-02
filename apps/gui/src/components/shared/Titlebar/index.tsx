import { getCurrentWindow } from '@tauri-apps/api/window'

import './styles.css'
import { useTabs } from '../../../contexts/TabContext'
import getTitle from '../../../lib/getTitle'

export default function Titlebar() {
    const { tabs, activeTabId, openTab, closeTab, setActiveTab } = useTabs()

  return (
    <header id="titlebar" data-tauri-drag-region>
        <div className="titlebar-left" data-tauri-drag-region>
            <div className="window-controls">
                <button className="control-btn" onClick={() => getCurrentWindow().close()}>
                    <span className="material-symbols-rounded">close</span>
                </button>
                <button className="control-btn" onClick={() => getCurrentWindow().minimize()}>
                    <span className="material-symbols-rounded">remove</span>
                </button>
                <button className="control-btn" onClick={() => getCurrentWindow().toggleMaximize()}>
                    <span className="material-symbols-rounded">crop_5_4</span>
                </button>
            </div>
            <button className='settings-btn'>
                <span className="material-symbols-rounded">settings</span>
            </button>
        </div>
        <div className="titlebar-right" data-tauri-drag-region>
            <div className="titlebar-tabs" data-tauri-drag-region>
                {tabs.map(tab => (
                    <button
                        key={tab.id}
                        className={`tab${tab.id === activeTabId ? " active" : ""}`}
                        onClick={() => setActiveTab(tab.id)}
                    >
                        <div className="tab-icon">
                            <span className="material-symbols-rounded">
                                {tab.location === "virtual://home" ? "home" : "folder"}
                            </span>
                        </div>
                        <div className="tab-title">{getTitle(tab.location)}</div>
                        <span
                            className="tab-close"
                            onClick={(e) => {
                                e.stopPropagation()
                                closeTab(tab.id)
                            }}
                        >
                            <span className="material-symbols-rounded">close</span>
                        </span>
                    </button>
                ))}
            </div>
            <button className="titlebar-add" onClick={() => openTab("virtual://home")}>
                <span className="material-symbols-rounded">add</span>
            </button>
        </div>
    </header>
  )
}