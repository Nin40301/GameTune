import {
  House,
  SlidersHorizontal,
  PackageOpen,
  ListChecks,
  History,
  ChartNoAxesCombined,
  Gamepad2,
  Settings2,
  ArrowUpRight,
  ArrowRight,
  ShieldCheck,
  ShieldAlert,
  TriangleAlert,
  Check,
  Plus,
  Monitor,
  Search,
  X,
  Layers,
  MemoryStick,
  Cpu,
  Power,
  FileCog,
  Blocks,
  AppWindow,
  SquareTerminal,
} from 'lucide-react'
const icons = {
  home: House,
  items: SlidersHorizontal,
  apps: PackageOpen,
  review: ListChecks,
  history: History,
  benchmark: ChartNoAxesCombined,
  games: Gamepad2,
  settings: Settings2,
  arrowUp: ArrowUpRight,
  arrowRight: ArrowRight,
  safe: ShieldCheck,
  high: ShieldAlert,
  warning: TriangleAlert,
  check: Check,
  plus: Plus,
  monitor: Monitor,
  search: Search,
  close: X,
  layers: Layers,
  memory: MemoryStick,
  cpu: Cpu,
  power: Power,
  registry: FileCog,
  minecraft: Blocks,
  app: AppWindow,
  service: SquareTerminal,
}
export function Icon({ name, className }: { name: keyof typeof icons; className?: string }) {
  const Component = icons[name]
  return (
    <Component
      size={20}
      strokeWidth={1.75}
      aria-hidden="true"
      focusable="false"
      className={className}
    />
  )
}
export function BrandMark() {
  return (
    <svg
      viewBox="0 0 40 44"
      width="36"
      height="40"
      aria-hidden="true"
      focusable="false"
      className="brand-mark"
    >
      <path
        d="M20 2 36 13v19L20 42 4 32V13Z"
        fill="#7c3aed"
        stroke="#6d28d9"
        strokeWidth="1.75"
        strokeLinejoin="round"
      />
      <path d="m20 2 7 14-7 26-7-26Z" fill="#c4b0ff" />
      <path d="m4 13 9 3 7 26L4 32ZM27 16l9-3v19L20 42Z" fill="#6d28d9" />
      <path d="m4 13 16 5 16-5M20 2v16" fill="none" stroke="#ebe3ff" strokeWidth="1.75" />
    </svg>
  )
}
