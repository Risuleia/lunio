import { ExplorerItem } from "../constants/ExplorerItem";

const EXT_ICONS: Record<string, string> = {
    // images
    png: "image",
    jpg: "image",
    jpeg: "image",
    webp: "image",
    bmp: "image",
    gif: "gif_box",
    tiff: "image",
    tif: "image",
    svg: "draw",
    heic: "image",
    raw: "photo_camera",
    psd: "brush",
    ai: "design_services",
    ico: "image",

    // video
    mp4: "video_file",
    mkv: "video_file",
    avi: "video_file",
    mov: "video_file",
    flv: "video_file",
    wmv: "video_file",
    webm: "video_file",
    mpeg: "video_file",
    mpg: "video_file",
    m4v: "video_file",

    // audio
    mp3: "music_note",
    wav: "music_note",
    ogg: "music_note",
    flac: "music_note",
    aac: "music_note",
    m4a: "music_note",
    wma: "music_note",
    mid: "music_note",

    // documents
    txt: "description",
    md: "description",
    rtf: "article",
    pdf: "picture_as_pdf",
    epub: "menu_book",
    mobi: "menu_book",
    doc: "article",
    docx: "article",
    odt: "article",
    pages: "article",
    tex: "functions",
    log: "subject",

    // spreadsheets
    xls: "grid_on",
    xlsx: "grid_on",
    ods: "grid_on",
    csv: "table_chart",
    tsv: "table_chart",
    numbers: "grid_on",

    // presentations
    ppt: "slideshow",
    pptx: "slideshow",
    odp: "slideshow",

    // archives
    zip: "folder_zip",
    rar: "folder_zip",
    "7z": "folder_zip",
    tar: "folder_zip",
    gz: "folder_zip",
    bz2: "folder_zip",
    xz: "folder_zip",
    zst: "folder_zip",
    cab: "folder_zip",
    arj: "folder_zip",
    lzh: "folder_zip",

    iso: "disc_full",

    // source code (web)
    html: "code",
    css: "palette",
    js: "code",
    jsx: "code",
    ts: "code",
    tsx: "code",
    json: "data_object",
    yaml: "settings",
    yml: "settings",
    xml: "code",
    env: "lock",

    // programming languages
    rs: "terminal",
    py: "terminal",
    java: "terminal",
    kt: "terminal",
    go: "terminal",
    c: "terminal",
    cpp: "terminal",
    h: "terminal",
    cs: "terminal",
    swift: "terminal",
    rb: "terminal",
    php: "terminal",
    scala: "terminal",
    lua: "terminal",
    r: "terminal",
    m: "terminal",
    asm: "terminal",

    // config & data
    ini: "settings",
    toml: "settings",
    conf: "settings",
    cfg: "settings",
    lock: "lock",
    cache: "storage",
    db: "storage",
    sqlite: "storage",
    sqlite3: "storage",
    sql: "storage",
    bak: "restore",
    tmp: "history_toggle_off",

    // executables and installers
    exe: "smart_toy",
    msi: "apps",
    app: "apps",
    jar: "apps",
    apk: "android",
    aab: "android",
    sh: "terminal",
    bat: "terminal",
    cmd: "terminal",
    run: "terminal",
    bin: "memory",
    deb: "apps",
    rpm: "apps",
    pkg: "apps",

    // dev project files
    gradle: "build",
    pom: "build",
    make: "build",
    cmake: "build",
    vs: "settings",
    idea: "settings",
    project: "settings",
    sln: "settings",
    workspace: "settings",

    // web assets
    wasm: "integration_instructions",
    map: "map",
    htaccess: "security",
    htpasswd: "lock",
    webmanifest: "manifest",
    serviceworker: "cached",

    // security & certificates
    key: "vpn_key",
    pem: "verified_user",
    crt: "verified_user",
    cert: "verified_user",
    pfx: "security",
    der: "verified_user",
    sig: "gpp_good",
    asc: "gpp_good",

    // design / 3d / media
    blend: "view_in_ar",
    fbx: "view_in_ar",
    obj: "view_in_ar",
    stl: "view_in_ar",
    sketch: "draw",
    figma: "design_services",
    xcf: "brush",
    cdr: "brush",
    "3ds": "view_in_ar",
    max: "view_in_ar",

    // game files
    unity: "sports_esports",
    asset: "inventory",
    pak: "inventory",
    rom: "sports_esports",
    sav: "save",
    mod: "extension",

    // log & debug
    dmp: "bug_report",
    trace: "timeline",
    stacktrace: "bug_report",
    crash: "dangerous",

    // backups & virtual
    vmdk: "dns",
    vdi: "dns",
    vhd: "dns",
    ovf: "dns",
    img: "dns",
    swp: "restore",

    // misc
    torrent: "cloud_download",
    eml: "mail",
    msg: "mail",
    wlmp: "movie",
    ics: "event",
    opml: "rss_feed",
    nfo: "info",
    pkpass: "credit_card",
    rss: "rss_feed",
}

export function getFileIcon(item: ExplorerItem): string {
    if (item.isDir) return "folder"

    const ext = item.ext?.toLowerCase()
    if (ext && ext in EXT_ICONS) return EXT_ICONS[ext]

    return "insert_drive_file"
}