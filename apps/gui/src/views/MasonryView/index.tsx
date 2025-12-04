import { useEffect, useMemo, useRef, useState } from "react";
import "./styles.css";
import GridView from "../GridView";
import { ExplorerItem } from "../../constants/ExplorerItem";
import useThumbnail from "../../hooks/useThumbnail";
import { openFile } from "../../services/daemon";
import { useTabs } from "../../contexts/TabContext";

type Column = ExplorerItem[];

const COLUMN_WIDTH = 200
const GAP = 16

function isMedia(item: ExplorerItem) {
	return (
		!item.isDir &&
		!!item.ext &&
		/png|jpe?g|gif|bmp|webp|tiff|tif|mp4|mkv|mov|avi|webm|flv|wmv/i.test(item.ext)
	);
}

export default function MasonryView({
	entries,
	register
}: {
	entries: ExplorerItem[],
	register: (id: string, el: HTMLElement | null) => void
}) {
	const [columns, setColumns] = useState<Column[]>([])

	const folders = useMemo(() => entries.filter(e => e.isDir), [entries])
	const media = useMemo(() => entries.filter(isMedia), [entries])
	const files = useMemo(() => entries.filter(e => !e.isDir && !isMedia(e)), [entries])

	const containerRef = useRef<HTMLDivElement>(null)

	const { setRenderOrder } = useTabs()


	useEffect(() => {
		setRenderOrder(media.map(i => i.id))
	}, [entries])

	useEffect(() => {
		function layout() {
			if (!containerRef.current) return;

			const width = containerRef.current.clientWidth;
			const count = Math.max(1, Math.floor(width / (COLUMN_WIDTH + GAP)));

			const cols: Column[] = Array.from({ length: count }, () => []);
			const heights = new Array(count).fill(0);

			for (const file of media) {
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
	}, [entries]);

	return (
		<>
			<div
				className="masonry-view"
				ref={containerRef}
			>
				{columns.map((col, i) => (
					<div key={i} className="masonry-column">
						{col.map(item => (
							<MasonryTile
								key={item.id}
								register={register}
								item={item}
							/>
						))}
					</div>
				))}
			</div>

			{folders.length > 0 && <GridView entries={folders} register={register} />}

			{files.length > 0 && <GridView entries={files} register={register} />}
		</>
	);
}

function MasonryTile({
	item,
	register
}: {
	item: ExplorerItem,
	register: (id: string, el: HTMLElement | null) => void
}) {
	const thumb = useThumbnail(item.id)

	const {
		selectSingle,
		toggleSelect,
		rangeSelect,
		getActiveTab
	} = useTabs()

	const selection = getActiveTab().selection
	const renderOrder = getActiveTab().renderOrder
	const selected = selection.includes(item.id)

	return (
		<button
			ref={(e) => register(item.id, e)}
			className="masonry-tile"
			onDoubleClick={() => openFile(item.path)}
			data-selected={selected}
			data-item
			onClick={(e) => {
				e.stopPropagation()

				if (e.shiftKey && selection.length)
					rangeSelect(selection[0], item.id, renderOrder)

				else if (e.ctrlKey || e.metaKey)
					toggleSelect(item.id)

				else
					selectSingle(item.id)
			}}
		>
			{thumb ? (
				<img src={thumb} loading="lazy" decoding="async" />
			) : (
				<div className="masonry-skeleton" />
			)}
			<div className="masonry-label">{item.name}</div>
		</button>
	);
}
