import Button from '../../components/ui/Button'
import Search from '../../components/ui/Search'
import { useTabs } from '../../contexts/TabContext'
import GridView from '../GridView'
import ListView from '../ListView'
import MasonryView from '../MasonryView'
import './styles.css'
import { useModal } from '../../contexts/ModalOverlay'
import GroupOverlay from '../../components/overlays/GroupOverlay'
import SortOverlay from '../../components/overlays/SortOverlay'
import useEntries from '../../hooks/useEntries'
import { useEffect, useRef, useState } from 'react'
import { openFile } from '../../services/daemon'
import { intersects } from '../../lib/selection'
import { Point, pointInPolygon } from '../../lib/lasso'
import Lasso from '../../components/ui/Lasso'

export default function Explorer() {
  const {
    getActiveTab,
    setViewMode,
    goBack,
    goForward,
    canGoBack,
    canGoForward,
    clearSelection,
    selectAll,
    navigate
  } = useTabs()

  const { showModal } = useModal()
  const tab = getActiveTab()
  const { entries, loading, refresh } = useEntries(tab)

  const itemRects = useRef<Map<string, DOMRect>>(new Map())
  const containerRef = useRef<HTMLDivElement>(null)

  const [box, setBox] = useState<null | { x1: number; y1: number; x2: number; y2: number }>(null)
  const [lasso, setLasso] = useState<null | Point[]>(null)
  const [underSelection, setUnderSelection] = useState<string[]>([])

  useEffect(() => {
    function handleKey(e: KeyboardEvent) {
      if (
        document.activeElement instanceof HTMLInputElement ||
        document.activeElement instanceof HTMLTextAreaElement
      ) return

      const isMeta = e.ctrlKey || e.metaKey

      if (isMeta && e.key.toLowerCase() === 'a') {
        e.preventDefault()
        selectAll(tab.renderOrder)
      }

      if (e.key === 'Escape') {
        e.preventDefault()
        clearSelection()
      }

      if (e.key === 'Enter' && tab.selection.length === 1) {
        const item = entries.find(e => e.id === tab.selection[0])
        if (!item) return

        item.isDir ? navigate(item.path) : openFile(item.path)
      }
    }

    window.addEventListener('keydown', handleKey)
    return () => window.removeEventListener('keydown', handleKey)
  }, [tab, entries])

  function startSelection(e: React.MouseEvent) {
    if (e.button !== 0) return
    if ((e.target as HTMLElement).closest('[data-item]')) return

    setUnderSelection([])

    if (e.altKey) {
      setLasso([{ x: e.clientX, y: e.clientY }])
    } else {
      setBox({
        x1: e.clientX,
        y1: e.clientY,
        x2: e.clientX,
        y2: e.clientY
      })
    }
  }

  useEffect(() => {
    if (!box && !lasso) return

    function move(e: MouseEvent) {
      if (box) setBox(b => b && { ...b, x2: e.clientX, y2: e.clientY })
      if (lasso) addPoint(e.clientX, e.clientY)
    }

    function end(e: MouseEvent) {
      finalizeSelection(e)
      setBox(null)
      setLasso(null)
    }

    const ctrl = new AbortController()
    window.addEventListener("mousemove", move, { signal: ctrl.signal })
    window.addEventListener("mouseup", end, { signal: ctrl.signal })
    return () => ctrl.abort()

  }, [box, lasso])

  function finalizeSelection(e: MouseEvent) {
    const hits: string[] = []

    if (box) {
      const r = new DOMRect(
        Math.min(box.x1, box.x2),
        Math.min(box.y1, box.y2),
        Math.abs(box.x2 - box.x1),
        Math.abs(box.y2 - box.y1)
      )

      itemRects.current.forEach((b, id) => {
        if (intersects(r, b)) hits.push(id)
      })
    }

    if (lasso) {
      itemRects.current.forEach((r, id) => {
        const cx = r.left + r.width / 2
        const cy = r.top + r.height / 2
        if (pointInPolygon({ x: cx, y: cy }, lasso)) hits.push(id)
      })
    }

    if (!hits.length) {
        if (lasso) setLasso(null)
        if (box) setBox(null)
        return
    }

    if (e.ctrlKey || e.metaKey) {
      setUnderSelection([...tab.selection, ...hits])
    } else {
      setUnderSelection(hits)
    }
  }

  function addPoint(x: number, y: number) {
    setLasso(prev => {
      if (!prev) return prev
      const last = prev.at(-1)
      if (last && Math.abs(last.x - x) < 4 && Math.abs(last.y - y) < 4) return prev
      return [...prev, { x, y }]
    })
  }

  function handleClick() {
    underSelection.length > 0 ? selectAll(underSelection) : clearSelection()
  }

  function registerRect(id: string, el: HTMLElement | null) {
        if (!el) return
        itemRects.current.set(id, el.getBoundingClientRect())
    }

  return (
    <div id="explorer">
      <div className="action-bar">
        <Button icon='chevron_left' disabled={!canGoBack()} func={goBack} />
        <Button icon='chevron_right' disabled={!canGoForward()} func={goForward} />
        <Button icon='sync' func={refresh} disabled={loading} data-loading={loading} />
        <Search />
        <button className="new-btn"><span className="material-symbols-rounded">add</span></button>

        <div className="btn-group">
          <Button icon='grid_view' activeOption={tab.viewMode === 'grid'} func={() => setViewMode("grid")} />
          <Button icon='lists' activeOption={tab.viewMode === 'list'} func={() => setViewMode("list")} />
          <Button icon='browse' activeOption={tab.viewMode === 'masonry'} func={() => setViewMode("masonry")} />
        </div>

        <div className="btn-group">
          <Button icon='swap_vert' func={() => showModal(<SortOverlay />)} />
          <Button icon='stacks' func={() => showModal(<GroupOverlay />)} />
        </div>

        <Button icon='more_vert' />
      </div>

      <div
        className="explorer-container"
        onMouseDown={startSelection}
        onClick={handleClick}
        ref={containerRef}
      >
        {getActiveTab().viewMode == "list" && <ListView entries={entries} register={registerRect} />}
        {getActiveTab().viewMode == "grid" && <GridView entries={entries} register={registerRect} />}
        {getActiveTab().viewMode == "masonry" && <MasonryView entries={entries} register={registerRect} />}

        {box && (
          <div
            className="selection-box"
            style={{
              left: Math.min(box.x1, box.x2),
              top: Math.min(box.y1, box.y2),
              width: Math.abs(box.x2 - box.x1),
              height: Math.abs(box.y2 - box.y1),
            }}
          />
        )}

        {lasso && <Lasso points={lasso} />}
      </div>
    </div>
  )
}