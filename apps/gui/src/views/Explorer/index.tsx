import { useEffect, useState } from 'react'
import Button from '../../components/ui/Button'
import Search from '../../components/ui/Search'
import { useTabs } from '../../contexts/TabContext'
import GridView from '../GridView'
import ListView from '../ListView'
import MasonryView from '../MasonryView'
import './styles.css'
import { listDir } from '../../services/daemon'
import { ExplorerItem } from '../../constants/ExplorerItem'
import { adaptEntry } from '../../lib/adapt'
import { useModal } from '../../contexts/ModalOverlay'
import GroupOverlay from '../../components/overlays/GroupOverlay'
import SortOverlay from '../../components/overlays/SortOverlay'

export default function Explorer() {
    const { getActiveTab, setViewMode, goBack, goForward, canGoBack, canGoForward } = useTabs()
    const { showModal } = useModal()
    const [entries, setEntries] = useState<ExplorerItem[]>([])

    const tab = getActiveTab()

    useEffect(() => {
        const location = tab.location
        if (location.startsWith("virtual://")) return

        console.log("[Explorer] requesting listDir:", location);

        listDir(location)
            .then(raw => setEntries(raw.map(adaptEntry)))
            .catch(console.error);
    }, [tab.location])

  return (
    <div id="explorer">
        <div className="action-bar">
            <Button
                icon='chevron_left'
                disabled={!canGoBack()}
                func={goBack}
            />
            <Button
                icon='chevron_right'
                disabled={!canGoForward()}
                func={goForward}
            />
            <Button
                icon='sync'
            />
            <Search />
            <button className="new-btn">
                <span className="material-symbols-rounded">add</span>
            </button>
            <div className="btn-group">
                <Button
                    icon='grid_view'
                    activeOption={tab.viewMode == 'grid'}
                    func={() => setViewMode("grid")}
                />
                <Button
                    icon='lists'
                    activeOption={tab.viewMode == 'list'}
                    func={() => setViewMode("list")}
                />
                <Button
                    icon='browse'
                    activeOption={tab.viewMode == 'masonry'}
                    func={() => setViewMode("masonry")}
                />
            </div>
            <div className="btn-group">
                <Button
                    icon='swap_vert'
                    func={() => showModal(<SortOverlay />)}
                />
                <Button
                    icon='stacks'
                    func={() => showModal(<GroupOverlay />)}
                />
            </div>
            <Button
                icon='more_vert'
            />
        </div>
        <div className="explorer-container">
            {getActiveTab().viewMode == "list" && <ListView entries={entries} />}
            {getActiveTab().viewMode == "grid" && <GridView entries={entries} />}
            {getActiveTab().viewMode == "masonry" && <MasonryView entries={entries} />}
        </div>
    </div>
  )
}
