import { Link } from "react-router-dom"

const RootView = () => {
    return (
        <>
        <div className="w-full bg-black flex flex-auto flex-col justify-between">
            <h1 className="font-sans text-9xl h-auto">
                krkrs
            </h1>
            <nav className="flex flex-row items-end relative h-full bottom-0">
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
