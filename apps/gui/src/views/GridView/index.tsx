import { useEffect, useState } from "react";
import { ExplorerItem } from "../../constants/ExplorerItem";
import { useTabs } from "../../contexts/TabContext";
import { groupItems } from "../../lib/grouping";

import "./styles.css";
import { Group } from "../../lib/grouping/types";
import { sortGroups } from "../../lib/sorting";
import { getFileIcon } from "../../lib/icons";
import useThumbnail from "../../hooks/useThumbnail";
import { openFile } from "../../services/daemon";

export default function GridView({
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
		setGroups(sortGroups(groupItems(entries, tab.groupMode), tab.sortMode, tab.sortOrder))
	}, [tab.sortMode, tab.sortOrder, tab.groupMode, entries])

	useEffect(() => {
		setRenderOrder(groups.flatMap(g => g.items.map(i => i.id)))
	}, [groups])

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
			data-folder={file.isDir}
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
