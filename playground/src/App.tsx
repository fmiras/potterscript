import { useState } from 'react'
import { parse_potterscript } from 'potterscript-wasm'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'

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

function App() {
  const [count, setCount] = useState(0)
  const [code] = useState(defaultCode)

  return (
    <>
      <div>
        <a href="https://vitejs.dev" target="_blank">
          <img src={viteLogo} className="logo" alt="Vite logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <h1>Vite + React</h1>
      <div className="card">
        <button onClick={() => setCount((count) => count + 1)}>count is {count}</button>
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
        <p>
          <strong>Code:</strong>
        </p>
        <pre>{code}</pre>
        <p>
          <strong>Output:</strong>
        </p>
        <pre>{parse_potterscript(code)}</pre>
      </div>
      <p className="read-the-docs">Click on the Vite and React logos to learn more</p>
    </>
  )
}

export default App
