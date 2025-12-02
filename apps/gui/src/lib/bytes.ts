export function bytesToDataUrl(bytes: number[]): string {
    const bin = Uint8Array.from(bytes)
    const base64 = btoa(
        bin.reduce((s, b) => s + String.fromCharCode(b), "")
    )

    return `data:image/webp;base64,${base64}`
}