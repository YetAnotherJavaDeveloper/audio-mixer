import { useCallback, useEffect, useRef, useState } from 'react';
import init from 'wasm-api';
import PlayerComponent, { type PlayerRef } from './components/AudioPlayer';
import AudioFileInput from './components/FileInput';
import { ThemeProvider } from './components/theme-provider';


const App = () => {

  const appTitle = 'Audio Mixer Web UI';

  const audioContext = new AudioContext();

  const playerRef = useRef<PlayerRef>(null);

  const [timer, setTimer] = useState(0);

  const [_arrayBuffer, setArrayBuffer] = useState<Uint8Array | null>(null);

  const handleFileChange = useCallback(async (file: File) => {
    if (playerRef.current) {
      playerRef.current.loadAudioFile(file);
      try {
        const arrayBuffer = await file.arrayBuffer();
        const view = new DataView(arrayBuffer);
        const cloneBuffer = new Float32Array(arrayBuffer, view.byteOffset, view.byteLength / Float32Array.BYTES_PER_ELEMENT);

        setArrayBuffer(new Uint8Array(arrayBuffer));
        const audioBuff = await audioContext.decodeAudioData(arrayBuffer);
        console.log('Buffer length:', arrayBuffer.byteLength);
        console.log('Float32Array length:', cloneBuffer);
        console.log('Audio buffer duration:', audioBuff.duration);
        console.log('Audio buffer sample rate:', audioBuff.sampleRate);
        console.log('Audio buffer number of channels:', audioBuff.numberOfChannels);

        const wasm = await init();
        const message = wasm.hello(); // <-- string direct
        console.log('WASM message:', message);
        const result = await wasm.computation(audioBuff);
        console.log('WASM computation result:', result);
      } catch (error) {
        console.error('Error reading file:', error);
      }
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