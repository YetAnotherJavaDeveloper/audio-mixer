import { useRef } from 'react';
import PlayerComponent, { type PlayerRef } from './components/AudioPlayer';
import { ThemeProvider } from './components/theme-provider';

const App = () => {

  const appTitle = 'Audio Mixer Web UI';

  const playerRef = useRef<PlayerRef>(null);

  return (
    <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
      <div className="App flex flex-col h-screen">
        <header className="App-header h-20">
          <h1 className="text-2xl font-bold mb-4">{appTitle}</h1>
        </header>
        <main className="flex-grow min-h-0 h-full items-center justify-center bg-gray-100 dark:bg-gray-900">
          <div className="flex flex-col items-center w-full h-full p-4">
            <PlayerComponent ref={playerRef} />
          </div>
        </main>
      </div>
    </ThemeProvider>
  );
}

export default App;