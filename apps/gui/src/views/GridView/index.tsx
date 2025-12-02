import { useEffect, useState } from "react";
import { ExplorerItem } from "../../constants/ExplorerItem";
import { useTabs } from "../../contexts/TabContext";
import { groupItems } from "../../lib/grouping";

import "./styles.css";
import { Group } from "../../lib/grouping/types";
import { sortGroups } from "../../lib/sorting";
import { getFileIcon } from "../../lib/icons";
import useThumbnail from "../../hooks/useThumbnail";

export default function GridView({ entries }: { entries: ExplorerItem[] }) {
  const { navigate, getActiveTab } = useTabs()
  
  const tab = getActiveTab()
  const [groups, setGroups] = useState<Group<ExplorerItem>[]>([])
  
  useEffect(() => {
    setGroups(sortGroups(groupItems(entries, tab.groupMode), tab.sortMode, tab.sortOrder))
  }, [tab.sortMode, tab.sortOrder, tab.groupMode, entries])

  return (
    <div className="grid-view">
      {groups.map(group => (
        <div key={group.label}>
          {group.label != "" && <div className="group-header">{group.label}</div>}
          <div className="grid-container">
            {group.items.map(file => (
              <GridTile
                key={file.id}
                file={file}
                onOpen={(f) => {
                  if (!f.isDir) return;
                  navigate(f.path);
                }}
              />
            ))}
          </div>
        </div>
      ))}
    </div>
  );
}

export function GridTile({
  file,
  onOpen
}: {
  file: ExplorerItem,
  onOpen: (file: ExplorerItem) => void
}) {
  const thumb = file.isDir ? null : useThumbnail(file.id)

  return (
    <button className="grid-tile" onDoubleClick={() => onOpen(file)}>
      <div className="tile-thumb">
        {thumb ? (
          <img src={thumb} className="thumb" loading="lazy" decoding="async" />
        ) : (
          <span className="material-symbols-rounded">
            {getFileIcon(file)}
          </span>
        )}
      </div>
      <div className="tile-name">
        {file.name}
      </div>
    </button>
  );
}
