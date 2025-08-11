import { ThemeProvider } from './components/theme-provider'
import { useCallback, useEffect, useRef, useState } from 'react';
import PlayerComponent, { type PlayerRef } from './components/AudioPlayer';
import AudioFileInput from './components/FileInput';

const App = () => {

  const appTitle = 'Audio Mixer Web UI';

  const playerRef = useRef<PlayerRef>(null);

  const [timer, setTimer] = useState(0);

  const handleFileChange = useCallback((file: File) => {
    if (playerRef.current) {
      playerRef.current.loadAudioFile(file);
    }
  }, []);

  const incrementTimer = useCallback(() => {
    setTimer((prev) => prev + 1);
    if (playerRef.current) {
      playerRef.current.setPosition(timer);
    }
  }, [timer]);

  useEffect(() => {
    if (!playerRef.current) return;
    const currentPlayer = playerRef.current;
    const state = currentPlayer.getState();
    if (state.isPlaying) {
      const interval = setInterval(incrementTimer, 1000);
      return () => clearInterval(interval);
    }
  }, [incrementTimer]);

  return (
    <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
      <div className="App">
        <header className="App-header">
          <h1 className="text-2xl font-bold mb-4">{appTitle}</h1>
        </header>
        <main className="flex flex-col items-center justify-center min-h-screen bg-gray-100 dark:bg-gray-900">
          <div className="flex flex-col items-center min-w-full min-h-screen p-4">
            <PlayerComponent ref={playerRef} />
            <div className="mt-4">
              <AudioFileInput onFileSelected={handleFileChange} />
            </div>
          </div>
        </main>
      </div>
    </ThemeProvider>
  );
}

export default App;