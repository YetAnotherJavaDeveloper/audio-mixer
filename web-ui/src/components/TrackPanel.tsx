import { useAudioEngineContext } from "@/lib/audio/engine.context";
import { useMemo } from "react";

export const TrackPanel = () => {
  const { trackState } = useAudioEngineContext();

  const trackIds = useMemo(() => trackState.trackIds, [trackState]);

  return (
    <div className="p-2">
      {trackIds.map((id) => (
        <div key={id} className="flex items-center gap-4">
          <span>{`Track n°` + id}</span>
        </div>
      ))}
    </div>
  );
};

export default TrackPanel;