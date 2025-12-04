import { useEffect, useState } from "react"
import { getThumbnail, requestThumbnail } from "../services/daemon"
import { bytesToDataUrl } from "../lib/bytes"

const CACHE = new Map<string, string>()
const INFLIGHT = new Set<string>()

const POLL_INTERVAL = 600
const MAX_RETRIES = 20

export default function useThumbnail(id: string, enabled = true): string | null {
    const [src, setSrc] = useState<string | null>(() => CACHE.get(id) ?? null)

    useEffect(() => {
        if (!enabled) return

        if (CACHE.has(id)) {
            setSrc(CACHE.get(id)!)
            return
        }

        let cancelled = false
        let retries = 0

        async function poll() {
            try {
                const bytes = await getThumbnail(id)
                console.log(bytes)

                if (bytes?.length) {
                    const url = bytesToDataUrl(bytes)
                    CACHE.set(id, url)
                    INFLIGHT.delete(id)
    
                    if (!cancelled) setSrc(url)
                    return
                }
            } catch {}

            if (++retries < MAX_RETRIES) {
                setTimeout(poll, POLL_INTERVAL)
            }
        }

        if (!INFLIGHT.has(id)) {
            INFLIGHT.add(id)
            requestThumbnail(id).catch(() => {})
        }

        poll()

        return () => {
            cancelled = true
        }
    }, [id, enabled])

    return src
}