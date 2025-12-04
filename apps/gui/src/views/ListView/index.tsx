import { useEffect, useState } from "react";
import { ExplorerItem } from "../../constants/ExplorerItem";
import { useTabs } from "../../contexts/TabContext";
import { groupItems } from "../../lib/grouping";
import "./styles.css";
import { Group } from "../../lib/grouping/types";
import { sortGroups } from "../../lib/sorting";
import { getFileIcon } from "../../lib/icons";
import { openFile } from "../../services/daemon";

export default function ListView({
  entries,
  register
}: {
  entries: ExplorerItem[],
  register: (id: string, el: HTMLElement | null) => void
}) {
  const { navigate, getActiveTab, setRenderOrder } = useTabs()
  
  const tab = getActiveTab()
  const [groups, setGroups] = useState<Group<ExplorerItem>[]>([])

  useEffect(() => {
    setRenderOrder(groups.flatMap(g => g.items.map(i => i.id)))
  }, [groups])
  
  useEffect(() => {
    setGroups(sortGroups(groupItems(entries, tab.groupMode), tab.sortMode, tab.sortOrder))
  }, [tab.sortMode, tab.sortOrder, tab.groupMode, entries])

  return (
    <div className="list-view">
      <div className="list-header">
        <span>Name</span>
        <span>Modified</span>
        <span>Type</span>
        <span>Size</span>
      </div>

      {groups.map(group => (
        <div key={group.label}>
          {group.label != "" && <div className="group-header">{group.label}</div>}
          {group.items.map(file => (
            <ListRow
              key={file.id}
              file={file}
              register={register}
              onOpen={(f) => {
                if (f.isDir) navigate(f.path);
                else openFile(f.path)
              }}
            />
          ))}
        </div>
      ))}
    </div>
  );
}

function ListRow({
  file,
  register,
  onOpen,
}: {
  file: ExplorerItem,
  register: (id: string, el: HTMLElement | null) => void
  onOpen: (file: ExplorerItem) => void,
}) {
  const {
    selectSingle,
    toggleSelect,
    rangeSelect,
    getActiveTab
  } = useTabs()

  const selection = getActiveTab().selection
  const renderOrder = getActiveTab().renderOrder
  const selected = selection.includes(file.id)

  return (
    <button
      ref={(e) => register(file.id, e)}
      className="list-row"
      onDoubleClick={() => onOpen(file)}
      data-selected={selected}
      data-item
      onClick={(e) => {
        e.stopPropagation()

        if (e.shiftKey && selection.length)
          rangeSelect(selection[0], file.id, renderOrder)

        else if (e.ctrlKey || e.metaKey)
          toggleSelect(file.id)

        else
          selectSingle(file.id)
      }}
    >
      <span className="file-name">
        <span className="material-symbols-rounded">
          {getFileIcon(file)}
        </span>
        <span>{file.name}</span>
      </span>
      <span>{file.modified ? new Date(file.modified * 1000).toLocaleDateString() : "—"}</span>
      <span>{file.isDir ? "Folder" : file.ext?.toUpperCase()}</span>
      <span>{file.size ? formatSize(file.size) : "—"}</span>
    </button>
  );
}

function formatSize(bytes: number) {
  const kb = bytes / 1024;
  return kb < 1024 ? `${kb.toFixed(1)} KB` : `${(kb / 1024).toFixed(1)} MB`;
}
