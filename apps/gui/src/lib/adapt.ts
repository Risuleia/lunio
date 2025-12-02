import { ExplorerItem } from "../constants/ExplorerItem";
import { FileEntry } from "../services/daemon";

export function adaptEntry(f: FileEntry): ExplorerItem {
    const parts = f.path.split(/[\\]/)
    const name = parts.at(-1)!
    const dot = name.lastIndexOf(".")

    return {
        id: f.id,
        path: f.path,
        name,
        isDir: f.is_dir,
        size: f.size,
        modified: f.modified ?? undefined,
        ext: dot > 0 ? name.slice(dot + 1).toLowerCase() : undefined,
        hasThumbnail: f.has_thumbnail
    }
}