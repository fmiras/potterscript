import './globals.css'
import { Ysabeau_SC } from 'next/font/google'
import { ThemeProvider } from '@/components/theme-provider'
import { Toaster } from '@/components/ui/toaster'

const ysabeuSC = Ysabeau_SC({
  subsets: ['latin'],
  weight: '400'
})

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body className={ysabeuSC.className}>
        <ThemeProvider
          attribute="class"
          defaultTheme="system"
          enableSystem
          themes={['light', 'dark', 'gryffindor', 'slytherin', 'hufflepuff', 'ravenclaw']}
        >
          <main>{children}</main>
          <Toaster />
        </ThemeProvider>
      </body>
    </html>
  )
}
