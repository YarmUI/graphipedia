import React from 'react'
import ReactDOM from 'react-dom/client'
import Routes from './client/routes/index.tsx'

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <Routes />
  </React.StrictMode>,
)
