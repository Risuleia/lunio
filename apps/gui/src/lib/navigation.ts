export default function joinPath(base: string, next: string) {
    if (!base.endsWith("/")) base += "/"
    return base + next
}