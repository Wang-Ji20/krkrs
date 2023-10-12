import { useState } from 'react'
import * as krkrs from 'krkrs';
import './playView.css'
import TextDisplay from './component/TextDisplay';
import ImageDisplay from './component/ImageDisplay';

function PlayView() {
  const [krkri, setKrkrs] = useState<krkrs.State>();
  const [text, setText] = useState(["not loaded"]);
  const [image, setImage] = useState("not loaded");

  async function loadKrkrs() {
    const k = await krkrs.State.new_from_web('lorerei.ks');
    setKrkrs(k);
    loadText();
    loadImage();
  }

  async function loadText() {
    if (!krkri) return;
    for (let index = 0; index < krkri.render_text_len(); index++) {
      text[index] = krkri.render_text(index);
    }
    setText(text);
  }

  async function loadImage() {
    if (!krkri) return;
    setImage(krkri.render_image(0));
  }

  loadKrkrs();
  const _next = () => {
    krkri?.eval_cmd(
      krkrs.Command.new_preceed()
    );
    loadText();
  }

  return (
    <>
      <div className="play-view">
        <ImageDisplay imageSrc={image} />
        <TextDisplay text={text} />
      </div>
    </>
  )
}

export default PlayView
