import { ExplorerItem } from "../../constants/ExplorerItem";
import { Group } from "../grouping/types";
import { naturalCompare } from "./naturalCompare";
import { SortMode, SortOrder } from "./types";

function sortItems(
    items: ExplorerItem[],
    mode: SortMode,
    order: SortOrder
): ExplorerItem[] {
    const sorted = [...items].sort((a, b) => {
        switch (mode) {
            case "name":
                return naturalCompare(a.name, b.name)
            case "date":
                return (a.modified || 0) - (b.modified || 0)
            case "size":
                return (a.size || 0) - (b.size || 0)
            case "type":
                return (a.ext || "").localeCompare(b.ext || "")
            default:
                return 0
        }
    })

    return order == "desc" ? sorted.reverse() : sorted
}

export function sortGroups(
    groups: Group<ExplorerItem>[],
    mode: SortMode,
    order: SortOrder
): Group<ExplorerItem>[] {
    return groups.map(group => ({
        ...group,
        items: sortItems(group.items, mode, order)
    }))
}