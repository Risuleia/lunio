export function intersects(a: DOMRect, b: DOMRect) {
    return !(
        b.right < a.left ||
        b.left > a.right ||
        b.bottom < a.top ||
        b.top > a.bottom
    )
}