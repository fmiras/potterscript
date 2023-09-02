import { ModeToggle } from './mode-toggle'

export function Header() {
  return (
    <div className="container flex flex-col items-start justify-between space-y-2 py-4 sm:flex-row sm:items-center sm:space-y-0 md:h-16">
      <h2 className="text-lg font-semibold w-full">PotterScript Playground</h2>
      <div className="ml-auto flex w-full space-x-2 sm:justify-end">
        <ModeToggle />
      </div>
    </div>
  )
}
