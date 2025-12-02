import { ExplorerItem } from "../../constants/ExplorerItem";
import { Group } from "./types";

function sizeGroup(bytes: number): string {
    if (bytes < 100 * 1024) return "Tiny"
    if (bytes < 1 * 1024 * 1024) return "Small";
    if (bytes < 100 * 1024 * 1024) return "Medium";
    if (bytes < 1024 * 1024 * 1024) return "Large";
    return "Huge"
}

export function groupBySize(items: ExplorerItem[]): Group<ExplorerItem>[] {
    const map = new Map<string, ExplorerItem[]>();
    const folders = []

    for (const item of items) {
        if (item.isDir) {
            folders.push(item)
            continue
        }
        
        const key = sizeGroup(item.size);
        const arr = map.get(key) || [];
        arr.push(item);
        map.set(key, arr);
    }

    if (folders.length > 0) map.set("Folder", folders)
    
    return [...map.entries()].map(([label, items]) => ({ label, items }));
}