import { ImageMeta } from "./ImageMeta";

export type MasonryItem =
  | { kind: "image"; id: string; name: string; width: number; height: number; src: string; modified: string }
  | { kind: "folder"; id: string; name: string; modified: string }
  | { kind: "file"; id: string; name: string; ext: string; modified: string };

export const MASONRY_ITEMS: MasonryItem[] = [
  folder("Raw Assets"),
  folder("Exports"),
  folder("Drafts"),

  file("notes.txt", "txt"),
  file("contract.pdf", "pdf"),
  file("archive.zip", "zip"),

  image("mountains.jpg", 1600, 1066),
  image("portrait1.jpg", 800, 1200),
  image("sunset.png", 1920, 1080),
  image("vertical_nature.jpg", 900, 1350),
  image("city.jpg", 1400, 875),
  image("macro_flower.jpg", 1000, 1500),
  image("forest.png", 1800, 1200),
  image("food.jpg", 1200, 800),
  image("night_sky.jpg", 1600, 900),
  image("model.jpg", 700, 1100),
  image("architecture.jpg", 1400, 2100),
  image("dog.jpg", 1280, 853),
  image("car.png", 1920, 1280),
  image("abstract.jpg", 1000, 1000),
  image("bridge.jpg", 1200, 675),
  image("beach.jpg", 1500, 1000),
  image("birds.jpg", 1000, 1400),
  image("waterfall.jpg", 1100, 1700),
  image("street.jpg", 1300, 800),
  image("clouds.jpg", 1600, 1000)
];

function folder(name: string): Extract<MasonryItem, { kind: "folder" }> {
  return {
    kind: "folder",
    id: crypto.randomUUID(),
    name,
    modified: randomDate()
  };
}

function file(name: string, ext: string): Extract<MasonryItem, { kind: "file" }> {
  return {
    kind: "file",
    id: crypto.randomUUID(),
    name,
    ext,
    modified: randomDate()
  };
}

function image(name: string, w: number, h: number): Extract<MasonryItem, { kind: "image" }> {
  return {
    kind: "image",
    id: crypto.randomUUID(),
    name,
    width: w,
    height: h,
    src: `https://picsum.photos/${w}/${h}?random=${Math.random()}`,
    modified: randomDate()
  };
}

function randomDate() {
  const now = Date.now();
  const past = now - Math.random() * 1000 * 60 * 60 * 24 * 180;
  return new Date(past).toISOString().slice(0, 10);
}
