export type GroupMode = 
    | "none"
    | "type"
    | "date"
    | "size"
    | "kind"
    | "ext";

export type Group<T> = {
    label: string;
    items: T[]
}