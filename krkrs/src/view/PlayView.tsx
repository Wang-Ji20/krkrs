import { useEffect, useState } from 'react'
import * as krkrs from 'krkrs';
import './playView.css'
import TextDisplay from './component/TextDisplay';
import ImageDisplay from './component/ImageDisplay';

function PlayView() {
  const [krkri, setKrkrs] = useState<krkrs.State>();
  const [_temp, setForceUpdate] = useState(true);

  async function initKrkrs() {
    if (krkri) {
      return;
    }
    const k = await krkrs.State.new_from_web('lorerei.ks');
    setKrkrs(k);
    loadAssets();
  }

  function loadText() {
    if (!krkri) return;
    const text: string[] = [];
    for (let index = 0; index < krkri.render_text_len(); index++) {
      text[index] = krkri.render_text(index);
    }
    return text;
  }

  function loadImage() {
    if (!krkri) return;
    return krkri.render_image(0);
  }

  function loadAssets() {
    loadText();
    loadImage();
  }

  initKrkrs();

  function forceUpdate() {
    setForceUpdate(!_temp);
  }

  useEffect(
    () => {
      const handleKey = (e: KeyboardEvent) => {
        console.log(e.key)
        krkri?.eval_cmd(e.key)
        forceUpdate();
      }
      document.addEventListener('keyup', handleKey);

      return () => { document.removeEventListener('keyup', handleKey) }
    }
  )

  return (
    <>
      <div className="play-view" onMouseDown={
        () => {
          krkri?.eval_cmd("MouseClick")
          forceUpdate();
        }}>
        <ImageDisplay imageSrc={loadImage() ?? 'unloaded'} />
        <TextDisplay text={loadText() ?? ['unloaded']} />
      </div>
    </>
  )
}

export default PlayView
