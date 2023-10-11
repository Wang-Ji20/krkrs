import { useState } from 'react'
import * as krkrs from 'krkrs';
import './playView.css'
import TextDisplay from './component/TextDisplay';
import ImageDisplay from './component/ImageDisplay';

function PlayView() {
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

  loadKrkrs();
  const _next = () => {
    if (!loaded) {
      return;
    }
    krkri?.eval_cmd(
      krkrs.Command.new_preceed()
    );
    resetText();
  }

  return (
    <>
      <div className="play-view">
        <ImageDisplay imageSrc={"/bgimage/o衛宮邸外観-(昼).png"} />
        <TextDisplay text={[
          "I go outside with Illya.",
          "We can’t spare the time to go shopping often, so we’ll have to push ourselves and buy about three days’ worth of groceries.",
        ]} />
      </div>
    </>
  )
}

export default PlayView
