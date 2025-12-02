import { ExplorerItem } from "../../constants/ExplorerItem";
import groupByDate from "./groupByDate";
import { groupByExt } from "./groupByExt";
import { groupByKind } from "./groupByKind";
import { groupBySize } from "./groupBySize";
import { groupByType } from "./groupByType";
import { Group, GroupMode } from "./types";

export function groupItems(
    items: ExplorerItem[],
    mode: GroupMode
): Group<ExplorerItem>[] {
    switch (mode) {
        case "type":
            return groupByType(items)
        case "date":
            return groupByDate(items)
        case "size":
            return groupBySize(items)
        case "kind":
            return groupByKind(items)
        case "ext":
            return groupByExt(items)
        default:
            return [{ label: "", items }]
    }
}