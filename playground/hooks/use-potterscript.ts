import { useEffect, useState } from 'react'

export function usePotterScript() {
  const [wasm, setWasm] = useState<typeof import('potterscript-wasm') | null>(null)
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    const loadWasm = async () => {
      const wasm = await import('potterscript-wasm')
      setWasm(wasm)
      setLoading(false)
    }

    loadWasm()
  }, [])
  return { wasm, loading }
}
