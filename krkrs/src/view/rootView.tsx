import { Link } from "react-router-dom"
import "./rootView.css"

const RootView = () => {
    return (
        <>
            <div className="w-full h-full bg-root flex flex-auto flex-col justify-between items-center">
                <h1 className="font-sans text-9xl title-color h-auto relative top-1/4">
                    krkrs
                </h1>
                <nav className="flex flex-row items-end relative bottom-1/4">
                    <Link to="/game">
                        <button className="bg-gray-800 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded">
                            Start game
                        </button>
                    </Link>
                </nav>
            </div>
        </>
    )
}

export default RootView;
