import { ExplorerItem } from "../../constants/ExplorerItem";
import { Group } from "./types";

export function groupByKind(items: ExplorerItem[]): Group<ExplorerItem>[] {
    const folders = items.filter(i => i.isDir)
    const files = items.filter(i => !i.isDir)

    const groups = []
    if (folders.length) groups.push({ label: "Folders", items: folders })
    if (files.length) groups.push({ label: "Files", items: files })

    return groups
}