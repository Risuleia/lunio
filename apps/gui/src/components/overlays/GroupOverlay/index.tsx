import { useModal } from '../../../contexts/ModalOverlay'
import { useTabs } from '../../../contexts/TabContext'
import { GroupMode } from '../../../lib/grouping/types'
import './styles.css'

const OPTIONS: { label: string, value: GroupMode, description: string, icon: string }[] = [
    { label: "None", value: "none", description: "No grouping applied", icon: "filter_none" },
    { label: "Type", value: "type", description: "Group by file type", icon: "unknown_document" },
    { label: "Date modified", value: "date", description: "Group by modification date", icon: "date_range" },
    { label: "Size", value: "size", description: "Group by size", icon: "responsive_layout" },
    { label: "Kind", value: "kind", description: "Group into files and folders", icon: "files" },
    { label: "Extension", value: "ext", description: "Group by file extension", icon: "extension" }
]

export default function GroupOverlay() {
    const { hideModal } = useModal()
    const { getActiveTab, setGroupMode } = useTabs()
    const active = getActiveTab().groupMode

    return (
        <section className="overlay-panel">
            <div className="overlay-header">
                <h2>Group Files</h2>
                <p>Organize files into categories</p>
            </div>
            <div className="overlay-list">
                {OPTIONS.map(o => (
                    <button
                        key={o.value}
                        className={`overlay-option${active === o.value ? " active" : ""}`}
                        onClick={() => {
                            setGroupMode(o.value)
                            hideModal()
                        }}
                    >
                        <span className="material-symbols-rounded">{o.icon}</span>
                        <div>
                            <span>{o.label}</span>
                            <span className='desc'>{o.description}</span>
                        </div>
                    </button>
                ))}
            </div>
            <div className="overlay-footer">
                <span className="material-symbols-rounded">fiber_manual_record</span>
                Click an option to apply grouping
            </div>
        </section>
    )
}
