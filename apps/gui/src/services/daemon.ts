import { invoke } from "@tauri-apps/api/core"

export type FileEntry = {
	id: string,
	path: string,
	size: number,
	is_dir: boolean,
	modified?: number,
	has_thumbnail: boolean
}

export async function connect() {
	return await invoke<void>("cmd_connect")
}

export async function search(query: string, limit?: number) {
	return await invoke<FileEntry[]>("cmd_search", { query, limit })
}

export async function listDir(path: string) {
	return await invoke<FileEntry[]>("cmd_list_dir", { path });
}

export async function requestThumbnail(id: string) {
	return await invoke<void>("cmd_request_thumbnail", { id });
}

export async function getThumbnail(id: string) {
	return await invoke<number[]>("cmd_get_thumbnail", { id });
}

export async function openFile(path: string) {
	return await invoke<void>("cmd_open_file", { path })
}

export async function shutdown() {
	return await invoke<void>("cmd_shutdown");
}


export type SystemEntry = {
	label: string,
	path: string,
	kind: "folder" | "drive",
	icon: string
}

export async function getSidebarEntries(): Promise<SystemEntry[]> {
	return await invoke<SystemEntry[]>("get_sidebar_entries")
}