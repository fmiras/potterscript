import { useState } from 'react'
import { parse_potterscript } from 'potterscript-wasm'
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
  const [code, setCode] = useState(defaultCode)
  const [ast, setAst] = useState(parse_potterscript(code))

  return (
    <>
      <h1>PotterScript Playground</h1>
      <div className="container">
        <div className="editor">
          <textarea
            value={code}
            onChange={(e) => {
              setCode(e.target.value)
              setAst(parse_potterscript(e.target.value))
            }}
          ></textarea>
        </div>
        <div className="ast">
          <pre>{JSON.stringify(ast, null, 2)}</pre>
        </div>
      </div>
    </>
  )
}

export default App
