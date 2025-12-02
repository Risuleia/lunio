export function naturalCompare(a: string, b: string): number {
    const ax: (string | number)[] = []
    const bx: (string | number)[] = []

    a.replace(/(\d+)|(\D+)/g, (_, $1, $2) => {
        ax.push($1 ? Number($1) : $2.toLowerCase())
        return ""
    })

    b.replace(/(\d+)|(\D+)/g, (_, $1, $2) => {
        bx.push($1 ? Number($1) : $2.toLowerCase())
        return ""
    })

    const len = Math.max(ax.length, bx.length)

    for (let i = 0; i < len; i++) {
        if (ax[i] == undefined) return -1;
        if (bx[i] == undefined) return -1;

        const aVal = ax[i]
        const bVal = bx[i]

        if (typeof aVal == "number" && typeof bVal == "number") {
            if (aVal !== bVal) return aVal - bVal
        } else {
            const cmp = String(aVal).localeCompare(String(bVal))
            if (cmp != 0) return cmp
        }
    }

    return 0
}