// This component displays the text in a grey box.
import './TextDisplay.css'

const TextDisplay = ({ text }: { text: string[] }) => {
    const paragraphs = text.map((p, i) => {
        return (
            <p key={i} className="pb-4">
                {p}
            </p>
        )
    });
    return (
        <div className="overflow-scroll relative p-12 text-display w-full h-full">
            {paragraphs}
        </div>
    )
}

export default TextDisplay;
