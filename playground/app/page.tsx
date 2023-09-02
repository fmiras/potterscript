import { Metadata } from 'next'

import { Separator } from '@/components/ui/separator'
import { Header } from '@/components/header'
import { Playground } from '@/components/playground'

export const metadata: Metadata = {
  title: 'PotterScript Playground',
  description: 'Try out PotterScript in the browser using a WASM build.'
}

export default function PlaygroundPage() {
  return (
    <>
      <div className="hidden h-full flex-col md:flex">
        <Header />
        <Separator />
        <Playground />
      </div>
    </>
  )
}
