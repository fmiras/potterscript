import { GithubIcon } from 'lucide-react'

import { Button } from '@/components/ui/button'

export function GithubButton() {
  return (
    <a href="https://github.com/fmiras/potterscript" target="_blank" rel="noopener noreferrer">
      <Button variant="outline" size="icon">
        <GithubIcon className="h-4 w-4" />
      </Button>
    </a>
  )
}
