// This component displays image in a stack manner
import './image-displayer.css'



const ImageDisplay = ({ imageSrc }: { imageSrc: string }) => {
    return (
        <img className="image-displayer" src={imageSrc} alt="background" />
    )
}

export default ImageDisplay;