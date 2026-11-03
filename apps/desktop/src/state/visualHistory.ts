type Point = { time: number; value: number }
// Visual history only. Rust collection and benchmark persistence are unchanged.
export function appendCpuSample(old: Point[], latest: Point): Point[] {
  if (old.at(-1)?.time === latest.time) return old
  return [...old.filter((point) => latest.time - point.time < 60000), latest]
}
