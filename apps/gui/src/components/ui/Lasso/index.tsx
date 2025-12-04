import { useEffect, useState } from 'react'
import { Point, savitzkyGolay, smoothen, smoothPath } from '../../../lib/lasso'

import './styles.css'

export default function Lasso({ points }: { points: Point[] }) {
    const [glowPoints, setGlow] = useState<Point[]>(points)
    const smoothPoints = smoothen(savitzkyGolay(points))

    useEffect(() => {
        const id = requestAnimationFrame(() => setGlow(smoothPoints.slice(0, -1)))

        return () => cancelAnimationFrame(id)
    }, [smoothPoints])

    return (
        <svg className='lasso-layer'>
            <path
                d={smoothPath(glowPoints)}
                className='lasso-glow'
            />
            <path
                d={smoothPath(smoothPoints)}
                className='lasso-path'
            />
        </svg>
    )
}
