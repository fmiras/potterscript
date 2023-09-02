'use client'
import { useCallback, useEffect, useState } from 'react'
import { Button } from './ui/button'
import { Textarea } from './ui/textarea'
import { usePotterScript } from '@/hooks/use-potterscript'
import { useToast } from './ui/use-toast'

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
      if (loading) return
      const ast = JSON.parse(wasm.parse_potterscript(code))[1]
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
    <div className="container h-full py-6">
      <div className="grid h-full items-stretch gap-6">
        <div className="md:order-1">
          <div className="mt-0 border-0 p-0">
            <div className="flex flex-col space-y-4">
              <div className="grid h-full grid-rows-2 gap-6 lg:grid-cols-2 lg:grid-rows-1">
                <div className="h-full">
                  <form
                    className="h-full space-y-5"
                    onSubmit={handleSubmit}
                    onKeyDown={handleKeyDown}
                  >
                    <Textarea
                      placeholder={defaultCode}
                      value={code}
                      onChange={(e) => setCode(e.target.value)}
                      className="h-full min-h-[300px] lg:min-h-[700px] xl:min-h-[700px]"
                    />
                    <div className="flex items-center space-x-2">
                      <Button type="submit">Parse</Button>
                      <span className="text-sm">or ⌘ + Enter</span>
                    </div>
                  </form>
                </div>
                <div className="space-y-5 h-full ">
                  <Textarea
                    className="rounded-md border bg-muted h-full min-h-[300px] lg:min-h-[700px] xl:min-h-[700px]"
                    readOnly
                    value={result}
                  />
                  <div className="flex items-center space-x-2">
                    <Button onClick={handleCopy}>Copy</Button>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}
