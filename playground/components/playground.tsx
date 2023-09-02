'use client'
import { useCallback, useEffect, useState } from 'react'
import { Button } from './ui/button'
import { Textarea } from './ui/textarea'

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

let parser: (code: string) => string

export function Playground() {
  const [code, setCode] = useState(defaultCode)
  const [result, setResult] = useState('')

  const handleParse = useCallback(() => {
    if (!parser) return
    const ast = JSON.parse(parser(code))[1]
    setResult(JSON.stringify(ast, null, 2))
  }, [code])

  // load WASM lib
  useEffect(() => {
    const loadWasm = async () => {
      const { parse_potterscript } = await import('potterscript-wasm')
      parser = parse_potterscript
      handleParse()
    }
    loadWasm()
  }, [code, handleParse])

  return (
    <div className="container h-full py-6">
      <div className="grid h-full items-stretch gap-6">
        <div className="md:order-1">
          <div className="mt-0 border-0 p-0">
            <div className="flex flex-col space-y-4">
              <div className="grid h-full grid-rows-2 gap-6 lg:grid-cols-2 lg:grid-rows-1">
                <Textarea
                  placeholder={defaultCode}
                  value={code}
                  onChange={(e) => setCode(e.target.value)}
                  className="h-full min-h-[300px] lg:min-h-[700px] xl:min-h-[700px]"
                />
                <div className="rounded-md border bg-muted">{result}</div>
              </div>
              <div className="flex items-center space-x-2">
                <Button onClick={handleParse}>Parse</Button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}
