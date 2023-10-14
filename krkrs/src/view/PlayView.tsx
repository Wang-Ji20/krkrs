import { useEffect, useState } from 'react'
import * as krkrs from 'krkrs';
import './playView.css'
import TextDisplay from './component/TextDisplay';
import ImageDisplay from './component/ImageDisplay';

type RenderContext = {
    'text': string[];
    'scene': string[];
}

function PlayView() {
    const [krkri, setKrkrs] = useState<krkrs.App>();
    const [text, setText] = useState(['unloaded']);
    const [image, setImage] = useState('unloaded');

    async function initKrkrs() {
        if (krkri) {
            return;
        }
        const k = await krkrs.App.new_web_from_url('lorerei.ks', (ctx: RenderContext) => {
            console.log('rendering');
            console.log(ctx);
            setText(ctx['text']);
            setImage(ctx['scene'][0]);
        });
        setKrkrs(k);
    }

    initKrkrs();

    useEffect(
        () => {
            const handleKey = (e: KeyboardEvent) => {
                krkri?.handle_web_input(e.key)
            }
            document.addEventListener('keyup', handleKey);

            return () => { document.removeEventListener('keyup', handleKey) }
        }
    )

    return (
        <>
            <div className="play-view" onMouseDown={
                (e) => {
                    e.preventDefault();
                    krkri?.handle_web_input("MouseClick")
                }}>
                <ImageDisplay imageSrc={image} />
                <TextDisplay text={text} />
            </div>
        </>
    )
}

export default PlayView
