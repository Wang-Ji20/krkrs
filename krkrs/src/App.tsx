import { useState } from 'react'
import './App.css'
import * as krkrs from 'krkrs';

function App() {
  const [krkri, setKrkrs] = useState<krkrs.State>();
  const [loaded, setLoaded] = useState(false);
  const [text, setText] = useState("not loaded");

  async function loadKrkrs() {
    const k = await krkrs.State.new_from_web('lorerei.ks');
    setKrkrs(k);
    setLoaded(true);
    setText(k.render());
  }

  const resetText = () => {
    if (loaded) setText(krkri!.render());
  }

  return (
    <>
      <button onClick={loadKrkrs}>
        Start game
      </button>
      <h1>krkrs</h1>
      <div className="card">
        <button onClick={() => {
          if (!loaded) {
            return;
          }
          krkri?.eval_cmd(
            krkrs.Command.new_preceed()
          );
          resetText();
        }}>
          next
        </button>
      </div>
      <p>
        {text}
      </p>
    </>
  )
}

export default App
