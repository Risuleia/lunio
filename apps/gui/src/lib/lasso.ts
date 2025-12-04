export type Point = {
    x: number,
    y: number
}

export function pointInPolygon(
    point: Point,
    poly: Point[]
): boolean {
    let inside = false

    for (let i = 0, j = poly.length - 1; i < poly.length; j = i++) {
        const xi = poly[i].x, yi = poly[i].y
        const xj = poly[j].x, yj = poly[j].y

        const intersect = 
            yi > point.y != yj > point.y &&
            point.x < (xj - xi) * (point.y - yi) / (yj - yi) + xi
        
        if (intersect) inside = !inside
    }

    return inside
}

function mid(a: Point, b: Point): Point {
    return {
        x: (a.x + b.x) / 2,
        y: (a.y + b.y) / 2
    }
}

export function smoothPath(points: Point[]) {
    if (points.length < 2) return ""

    let d = `M ${points[0].x} ${points[0].y}`

    for (let i = 0; i < points.length - 1; i++) {
        const midPoint = mid(points[i], points[i + 1])

        d += `Q ${points[i].x} ${points[i].y} ${midPoint.x},${midPoint.y}`
    }

    const last = points[points.length - 1]
    d += `T ${last.x},${last.y}`

    return d
}

export function smoothen(points: Point[]): Point[] {
  if (points.length < 3) return points

  const out: Point[] = [points[0]]

  for (let i = 1; i < points.length - 1; i++) {
    const prev = points[i - 1]
    const curr = points[i]
    const next = points[i + 1]

    out.push({
      x: curr.x * 0.5 + (prev.x + next.x) * 0.25,
      y: curr.y * 0.5 + (prev.y + next.y) * 0.25
    })
  }

  out.push(points[points.length - 1])
  return out
}

export function catmullRom(points: Point[], tension: number = 0.5, steps: number = 8) {
    if (points.length < 4) return points

    const out: Point[] = []

    for (let i = 0; i < points.length - 3; i++) {
        const p0 = points[i]
        const p1 = points[i + 1]
        const p2 = points[i + 2]
        const p3 = points[i + 3]

        for (let t = 0; t <= steps; t++) {
            const s = t / steps
            const s2 = s * s
            const s3 = s2 * s

            const x =
                (-tension*s3 + 2*tension*s2 - tension*s) * p0.x +
                ((2-tension)*s3 + (tension-3)*s2 + 1) * p1.x +
                ((tension-2)*s3 + (3-2*tension)*s2 + tension*s) * p2.x +
                (tension*s3 - tension*s2) * p3.x

            const y =
                (-tension*s3 + 2*tension*s2 - tension*s) * p0.y +
                ((2-tension)*s3 + (tension-3)*s2 + 1) * p1.y +
                ((tension-2)*s3 + (3-2*tension)*s2 + tension*s) * p2.y +
                (tension*s3 - tension*s2) * p3.y

            out.push({ x, y })
        }
    }

    return out
}

export function savitzkyGolay(points: Point[], windowSize: number = 3, order: number = 2): Point[] {
  const coefficients = getSavitzkyGolayCoefficients(windowSize, order);
  const halfWindow = Math.floor(windowSize / 2);

  return points.map((point, i) => {
    // If we're at the edges, just return the point itself to avoid the weird line.
    if (i < halfWindow || i >= points.length - halfWindow) {
      return point;
    }

    let y = 0;
    for (let j = -halfWindow; j <= halfWindow; j++) {
      const k = i + j;
      y += points[k].y * coefficients[j + halfWindow];
    }

    return { x: point.x, y };
  });
}

function getSavitzkyGolayCoefficients(windowSize: number, order: number): number[] {
  // For a window size of 5 and a quadratic polynomial (order 2)
  if (windowSize === 3 && order === 2) {
    return [1/6, 2/3, 1/6];
  }
  // You can add more precomputed coefficients for other window sizes and orders if needed.
  
  throw new Error('Unsupported window size or order');
}