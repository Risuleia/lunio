import { useTabs } from '../../../contexts/TabContext'
import { SortMode } from '../../../lib/sorting/types'
import './styles.css'

const OPTIONS: { label: string, value: SortMode, icon: string }[] = [
    { label: "Name", value: "name", icon: "sort_by_alpha" },
    { label: "Date", value: "date", icon: "event_upcoming" },
    { label: "Size", value: "size", icon: "align_vertical_top" },
    { label: "Type", value: "type", icon: "file_export" },
]

export default function SortOverlay() {
    const { getActiveTab, setSortMode, setSortOrder } = useTabs()
    const sortMode = getActiveTab().sortMode
    const sortOrder = getActiveTab().sortOrder

    function handleClick(o: typeof OPTIONS[number]) {
        if (o.value == sortMode) return setSortOrder(sortOrder == "asc" ? "desc" : "asc")
        setSortMode(o.value)
        setSortOrder("asc")
    }

    return (
        <section className="overlay-panel">
            <div className="overlay-header">
                <h2>Sort Files</h2>
                <p>Choose how to organize your files</p>
            </div>
            <div className="overlay-list">
                {OPTIONS.map(o => (
                    <button
                        key={o.value}
                        className={`overlay-option${sortMode === o.value ? " active" : ""}`}
                        onClick={() => handleClick(o)}
                    >
                        <span className="material-symbols-rounded">{o.icon}</span>
                        <div>
                            <span>{o.label}</span>
                            {sortMode == o.value && <span className='desc'>{sortOrder == "asc" ? "Ascending" : "Descending"}</span>}
                        </div>
                    </button>
                ))}
            </div>
            <div className="overlay-footer">
                <span className="material-symbols-rounded">fiber_manual_record</span>
                Click active option to switch sorting order
            </div>
        </section>
    )
}
