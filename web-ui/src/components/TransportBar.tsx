import { useCallback } from "react";
import { Button } from "./ui/button";
import { Pause, Play, Redo2, Undo2 } from "lucide-react";
import { useAudioEngineContext } from "@/lib/audio/engine.context";

const DEFAULT_TIME_JUMP = 5; // seconds

export const TransportBar: React.FC = () => {
  const { play, pause, isPlaying, seek, getPlayhead } = useAudioEngineContext();

  const togglePlay = useCallback(async (e: React.MouseEvent<HTMLButtonElement>) => {
    e.stopPropagation();
    e.preventDefault();
    console.info('Toggling play/pause');
    if (isPlaying()) await pause();
    else await play();
  }, [isPlaying, play, pause]);

  const seekTo = useCallback((seconds: number) => {
    const currentPlayhead = getPlayhead();
    const newFrame = currentPlayhead + seconds * 44100; // Assuming 44.1kHz
    seek(newFrame);
  }, [seek, getPlayhead]);

  return (
    <div className="flex items-center space-x-2">
      <Button size="sm" onClick={() => seekTo(-DEFAULT_TIME_JUMP)}>
        <Undo2 className="h-4 w-4" />
      </Button>
      <Button size="sm" onClick={togglePlay}>
        {isPlaying() ? <Pause className="h-4 w-4" /> : <Play className="h-4 w-4" />}
      </Button>
      <Button size="sm" onClick={() => seekTo(DEFAULT_TIME_JUMP)}>
        <Redo2 className="h-4 w-4" />
      </Button>
    </div>
  );
};

export default TransportBar;