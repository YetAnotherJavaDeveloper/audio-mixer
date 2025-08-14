import React from "react";
import { AudioEngineContext } from "./engine.context";
import { useAudioEngine } from "./engine.hook";

export const AudioEngineProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const engine = useAudioEngine(); // ton hook qui instancie le moteur

  return (
    <AudioEngineContext.Provider value={engine}>
      {children}
    </AudioEngineContext.Provider>
  );
};