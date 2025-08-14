import { AudioEngineProvider } from "@/lib/audio/engine.provider";
import TransportBar from "@/components/TransportBar";
import TrackPanel from "@/components/TrackPanel";

export const RootLayout: React.FC = () => {
  return (
    <AudioEngineProvider>
      <div>
        <TransportBar />
        <TrackPanel />
      </div>
    </AudioEngineProvider>
  );
}