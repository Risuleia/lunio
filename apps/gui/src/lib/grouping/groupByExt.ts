import { ExplorerItem } from "../../constants/ExplorerItem";
import { Group } from "./types";

export function groupByExt(items: ExplorerItem[]): Group<ExplorerItem>[] {
    const map = new Map<string, ExplorerItem[]>();
    const folders = []

    for (const item of items) {
        if (item.isDir) {
            folders.push(item)
            continue
        }

        const key = item.ext || "Other";
        const bucket = map.get(key) || []
        bucket.push(item)
        map.set(key, bucket)
    }

    if (folders.length > 0) map.set("Folder", folders)

    return [...map.entries()].map(([label, items]) => ({
        label: label == "Folder" ? label : `.${label}`,
        items
    }))
}