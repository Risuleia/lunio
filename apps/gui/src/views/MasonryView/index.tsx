import { useEffect, useMemo, useRef, useState } from "react";
import "./styles.css";
import GridView from "../GridView";
import { ExplorerItem } from "../../constants/ExplorerItem";
import useThumbnail from "../../hooks/useThumbnail";

type Column = ExplorerItem[];

const COLUMN_WIDTH = 200
const GAP = 16

function isImage(item: ExplorerItem) {
  return (
    !item.isDir &&
    !!item.ext &&
    /png|jpe?g|gif|bmp|webp|tiff/i.test(item.ext)
  );
}

export default function MasonryView({ entries }: { entries: ExplorerItem[] }) {
  const ref = useRef<HTMLDivElement>(null)
  const [columns, setColumns] = useState<Column[]>([])
  
  const folders = useMemo(() => entries.filter(e => e.isDir), [entries])
  const images = useMemo(() => entries.filter(isImage), [entries])
  const files = useMemo(() => entries.filter(e => !e.isDir && !isImage(e)), [entries]) 

  useEffect(() => {
    function layout() {
      if (!ref.current) return;

      const width = ref.current.clientWidth;
      const count = Math.max(1, Math.floor(width / (COLUMN_WIDTH + GAP)));

      const cols: Column[] = Array.from({ length: count }, () => []);
      const heights = new Array(count).fill(0);

      for (const file of images) {
        // fallback ratio → near-square
        const ratio = 1.2;
        const height = COLUMN_WIDTH * ratio;

        const idx = heights.indexOf(Math.min(...heights));
        heights[idx] += height + GAP;
        cols[idx].push(file);
      }

      setColumns(cols);
    }

    layout();
    window.addEventListener("resize", layout);

    return () => window.removeEventListener("resize", layout);
  }, []);

  return (
    <>
      <div className="masonry-view" ref={ref}>
        {columns.map((col, i) => (
          <div key={i} className="masonry-column">
            {col.map(item => (
              <MasonryTile key={item.id} item={item} />
            ))}
          </div>
        ))}
      </div>
      
      {folders.length > 0 && <GridView entries={folders} />}
          
      {files.length > 0 && <GridView entries={files} />}
    </>
  );
}

function MasonryTile({ item }: { item: ExplorerItem }) {
  const thumb = useThumbnail(item.id)

  return (
    <button className="masonry-tile">
      {thumb ? (
        <img src={thumb} loading="lazy" decoding="async" />
      ) : (
        <div className="masonry-skeleton" />
      )}
      <div className="masonry-label">{item.name}</div>
    </button>
  );
}
