import * as wasm from 'macc-bindings';

export default function Home() {
  return (
    <div>
      <h1>{wasm.greet('test')}</h1>
    </div>
  )
}
