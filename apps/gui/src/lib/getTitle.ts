export default function getTitle(location: string): string {
    // if (location.startsWith("virtual://") || location.startsWith("drive://")) {
        const name = location.replace(/(virtual:\/\/|drive:\/\/)/, "");
    // }

    const parts = name.split(/[\\/]/);
    return parts.at(-1) || "Tab";
}