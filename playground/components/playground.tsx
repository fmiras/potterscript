'use client'

import { useCallback, useEffect, useState } from 'react'
import { Button } from '@/components/ui/button'
import { Textarea } from '@/components/ui/textarea'
import { usePotterScript } from '@/hooks/use-potterscript'
import { useToast } from '@/components/ui/use-toast'
import { ConsoleAlert } from './console-alert'

const defaultCode = `index = 0

quidditch {
  snake = ~Serpensortia
  ~WingardiumLeviosa snake
  ~WingardiumLeviosa snake
  snake = snake + " some string"
  ~Revelio snake
  ~Incendio snake
  ~Revelio snake
  ~Engorgio index

  if index == 4 {
    snitch # Break loop
  }
}`

export function Playground() {
  const [code, setCode] = useState(defaultCode)
  const [result, setResult] = useState('')
  const { wasm, loading } = usePotterScript()
  const { toast } = useToast()

  const handleSubmit = useCallback(
    (e?: React.FormEvent<HTMLFormElement>) => {
      if (e) e.preventDefault()
      if (loading || !wasm) return

      const ast = JSON.parse(wasm.parse(code))
      wasm.parse_and_run(code)

      setResult(JSON.stringify(ast, null, 2))
    },
    [wasm, code, loading]
  )

  const handleCopy = useCallback(() => {
    navigator.clipboard.writeText(result)
    toast({
      title: 'Copied to clipboard'
    })
  }, [result, toast])

  const handleKeyDown = (event: React.KeyboardEvent<HTMLFormElement>) => {
    if ((event.metaKey || event.ctrlKey) && event.key === 'Enter') {
      handleSubmit(event)
    }
  }

  useEffect(() => {
    if (loading) return
    handleSubmit()
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [loading])

  return (
    <div className="flex flex-col container space-y-2 h-screen lg:flex-grow lg:py-6 lg:space-y-4">
      <ConsoleAlert />

      <div className="flex flex-col flex-grow border-0 p-0 lg:grid lg:gap-6 lg:grid-cols-2">
        <form
          className="flex flex-col flex-grow space-y-5"
          onSubmit={handleSubmit}
          onKeyDown={handleKeyDown}
        >
          <h2 className="text-lg font-semibold">Code</h2>

          <Textarea
            placeholder={defaultCode}
            value={code}
            onChange={(e) => setCode(e.target.value)}
            className="flex-grow bg-muted font-mono"
          />
          <div className="flex items-center justify-end">
            <div className="flex items-center space-x-2">
              <Button className="block lg:hidden" type="submit">
                Parse
              </Button>
              <Button className="hidden lg:block" type="submit">
                Run
              </Button>
              <span className="hidden lg:block text-sm">or ⌘ + Enter</span>
            </div>
          </div>
        </form>

        <form className="flex flex-col flex-grow space-y-5">
          <h2 className="text-lg font-semibold">Abstract Syntax Tree (AST)</h2>
          <Textarea className="flex-grow bg-muted font-mono" readOnly value={result} />
          <div className="flex items-center justify-end">
            <Button onClick={handleCopy}>Copy</Button>
          </div>
        </form>
      </div>
    </div>
  )
}
