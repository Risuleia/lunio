export type ExplorerItem = {
    id: string;
    path: string,
    name: string;
    isDir: boolean;
    size: number;
    modified?: number;
    ext?: string;
    hasThumbnail: boolean
};