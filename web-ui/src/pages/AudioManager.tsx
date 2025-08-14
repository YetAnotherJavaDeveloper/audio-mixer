import AudioFileInput, { type FileInputRef } from "@/components/AudioFileInput";
import TrackPanel from "@/components/TrackPanel";
import TransportBar from "@/components/TransportBar";
import { Button } from "@/components/ui/button";
import { useAudioEngineContext } from "@/lib/audio/engine.context";
import { useCallback, useEffect, useRef } from "react";

export const AudioManager: React.FC = () => {
    const inputRef = useRef<FileInputRef>(null);

    const { startEngine, stopEngine, getPlayhead, trackState: { addTrack } } = useAudioEngineContext();

    const handleFileUpload = useCallback(async (file: File) => {
        if (!file) return;
        const arrayBuffer = await file.arrayBuffer();
        addTrack(arrayBuffer);
    }, [addTrack]);

    useEffect(() => {
        const interval = setInterval(() => {
            const playhead = getPlayhead();
            console.log(`Current playhead position: ${playhead}`);
        }, 1000); // Log every second

        return () => clearInterval(interval);
    }, [getPlayhead]);

    return (
        <div className="flex flex-col w-full h-full p-4">
            <TransportBar />
            <TrackPanel />
            <AudioFileInput onFileSelected={handleFileUpload} onClick={() => inputRef?.current?.reset()} />
            <Button
                className="mt-4"
                onClick={startEngine}
            >
                Start Engine
            </Button>
            <Button
                className="mt-2"
                onClick={stopEngine}
            >
                Stop Engine
            </Button>
        </div>
    )
};

export default AudioManager;