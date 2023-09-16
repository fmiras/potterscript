import { GithubButton } from '@/components/github-button'
import { ModeToggle } from '@/components/mode-toggle'

export function Header() {
  return (
    <div className="container flex justify-between space-x-2 py-4 md:h-16">
      <h2 className="text-lg font-semibold">PotterScript Playground</h2>
      <div className="flex justify-end space-x-2">
        <GithubButton />
        <ModeToggle />
      </div>
    </div>
  )
}
