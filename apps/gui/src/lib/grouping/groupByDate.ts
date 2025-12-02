import { ExplorerItem } from "../../constants/ExplorerItem";
import { Group } from "./types";

function dateGroup(epoch?: number): string {
    if (!epoch) return "Unknown"

    const days = Math.floor((Date.now() - epoch * 100) / 86400000)

    if (days == 0) return "Today"
    if (days == 1) return "Yesterday"
    if (days < 7) return "This week"
    if (days < 30) return "This month"
    if (days < 365) return "This year"
    return "Older"
}

export default function groupByDate(items: ExplorerItem[]): Group<ExplorerItem>[] {
    const map = new Map<string, ExplorerItem[]>()

    for (const item of items) {
        const label = dateGroup(item.modified)
        const bucket = map.get(label) || []
        bucket.push(item)
        map.set(label, bucket)
    }

    return [...map.entries()].map(([label, items]) => ({ label, items }))
}