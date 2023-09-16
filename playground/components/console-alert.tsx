import { Terminal } from 'lucide-react'

import { Alert, AlertTitle, AlertDescription } from './ui/alert'

export function ConsoleAlert() {
  return (
    <Alert className="font-mono hidden lg:block">
      <Terminal className="h-4 w-4" />
      <AlertTitle>Open Your Browser{`'`}s Console Tab</AlertTitle>
      <AlertDescription>
        The WASM-enabled runtime binds a {`"`}Revelio{`"`} spell to your browser{`'`}s{' '}
        <b>console.log</b>
      </AlertDescription>
    </Alert>
  )
}
