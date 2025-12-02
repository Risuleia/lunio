import { useEffect, useState } from "react";
import { ExplorerItem } from "../../constants/ExplorerItem";
import { useTabs } from "../../contexts/TabContext";
import { groupItems } from "../../lib/grouping";
import "./styles.css";
import { Group } from "../../lib/grouping/types";
import { sortGroups } from "../../lib/sorting";
import { getFileIcon } from "../../lib/icons";

export default function ListView({ entries }: { entries: ExplorerItem[] }) {
  const { navigate, getActiveTab } = useTabs()
  
  const tab = getActiveTab()
  const [groups, setGroups] = useState<Group<ExplorerItem>[]>([])
  
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
              onOpen={(f) => {
                if (!f.isDir) return;
                navigate(f.path);
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
  onOpen
}: {
  file: ExplorerItem,
  onOpen: (file: ExplorerItem) => void
}) {
  return (
    <button className="list-row" onDoubleClick={() => onOpen(file)}>
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
