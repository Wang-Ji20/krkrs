import { createBrowserRouter, RouterProvider } from 'react-router-dom'
import './index.css'
import PlayView from './view/PlayView.tsx'
import RootView from './view/rootView.tsx'
import React from 'react';

const router = createBrowserRouter([
  {
    path: "/",
    element: <RootView />
  },
  {
    path: "game",
    element: <PlayView />
  }
])


function App() {
  return (
    <React.StrictMode>
      <div className='w-screen h-screen app'>
        <RouterProvider router={router} />
      </div>
    </React.StrictMode>
  )
}

export default App
