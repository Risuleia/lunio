import React, { useEffect, useRef, useState } from "react";
import { ExplorerItem } from "../../constants/ExplorerItem";
import { useTabs } from "../../contexts/TabContext";
import { groupItems } from "../../lib/grouping";

import "./styles.css";
import { Group } from "../../lib/grouping/types";
import { sortGroups } from "../../lib/sorting";
import { getFileIcon } from "../../lib/icons";
import useThumbnail from "../../hooks/useThumbnail";
import { openFile } from "../../services/daemon";
import { intersects } from "../../lib/selection";

export default function GridView({
  entries,
  register
}: {
  entries: ExplorerItem[],
  register: (id: string, el: HTMLElement | null) => void
}) {
  const { navigate, getActiveTab, setRenderOrder, selectAll } = useTabs()

  const tab = getActiveTab()
  const [groups, setGroups] = useState<Group<ExplorerItem>[]>([])

  const itemRects = useRef<Map<string, DOMRect>>(new Map())

  const [drag, setDrag] = useState<null | {
    x1: number,
    x2: number,
    y1: number,
    y2: number
  }>(null)

  const containerRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    setGroups(sortGroups(groupItems(entries, tab.groupMode), tab.sortMode, tab.sortOrder))
  }, [tab.sortMode, tab.sortOrder, tab.groupMode, entries])

  useEffect(() => {
    setRenderOrder(groups.flatMap(g => g.items.map(i => i.id)))
  }, [groups])

  function startSelection(e: React.MouseEvent) {
    if (e.button != 0) return
    if ((e.target as HTMLElement).closest(".grid-tile")) return

    setDrag({
      x1: e.clientX,
      y1: e.clientY,
      x2: e.clientX,
      y2: e.clientY
    })
  }

  useEffect(() => {
    if (!drag) return

    function move(e: MouseEvent) {
      setDrag(d => d && { ...d, x2: e.clientX, y2: e.clientY })
    }

    function end(e: MouseEvent) {
      finalizeSelection(e)
      setDrag(null)
    }
    
    const ctrl = new AbortController()

    window.addEventListener("mousemove", move, { signal: ctrl.signal })
    window.addEventListener("mouseup", end, { signal: ctrl.signal })

    return () => ctrl.abort()
  }, [drag])

  function finalizeSelection(e: MouseEvent) {
    if (!drag) return

    const rect = new DOMRect(
      Math.min(drag.x1, drag.x2),
      Math.min(drag.y1, drag.y2),
      Math.abs(drag.x2 - drag.x1),
      Math.abs(drag.y2 - drag.y1),
    )

    const hits: string[] = []

    itemRects.current.forEach((r, id) => {
      if (intersects(rect, r)) hits.push(id)
    })

    if (!hits.length) return

    if (e.ctrlKey || e.metaKey) {
      const existing = tab.selection
      selectAll([...existing, ...hits])
    } else {
      console.log(hits)
      selectAll(hits)
    }
  }

  return (
    <div
      className="grid-view"
      ref={containerRef}
      onMouseDown={startSelection}
    >
      {groups.map(group => (
        <div key={group.label}>
          {group.label != "" && <div className="group-header">{group.label}</div>}
          <div className="grid-container">
            {group.items.map(file => (
              <GridTile
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
        </div>
      ))}
    </div>
  );
}

export function GridTile({
  file,
  register,
  onOpen,
}: {
  file: ExplorerItem,
  register: (id: string, el: HTMLElement | null) => void
  onOpen: (file: ExplorerItem) => void,
}) {
  const thumb = file.isDir ? null : useThumbnail(file.id)

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
      className="grid-tile"
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
