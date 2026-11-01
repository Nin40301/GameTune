import { createRoot } from 'react-dom/client'
import { App } from './App'
import { createBackend } from './backend'
import './styles.css'
const root = document.getElementById('root')
if (root) void createBackend().then((backend) => createRoot(root).render(<App backend={backend} />))
