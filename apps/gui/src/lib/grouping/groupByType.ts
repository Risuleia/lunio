import { ExplorerItem } from "../../constants/ExplorerItem";
import { Group } from "./types";

export function groupByType(files: ExplorerItem[]): Group<ExplorerItem>[] {
    const groups: Record<string, ExplorerItem[]> = {
        Folders: [],
        Images: [],
        Videos: [],
        Audio: [],
        Documents: [],
        Other: []
    }

    for (const file of files) {
        if (file.isDir) {
            groups.Folders.push(file)
            continue
        }

        const ext = file.ext?.toLowerCase() || "";

        if (["png","jpg","jpeg","webp","gif"].includes(ext)) groups.Images.push(file);
        else if (["mp4","mov","mkv","avi"].includes(ext)) groups.Videos.push(file);
        else if (["mp3","wav","flac"].includes(ext)) groups.Audio.push(file);
        else if (["pdf","doc","docx","txt"].includes(ext)) groups.Documents.push(file);
        else groups.Other.push(file);
    }

    return Object.entries(groups)
        .filter(([, items]) => items.length)
        .map(([label, items]) => ({ label, items }))
}