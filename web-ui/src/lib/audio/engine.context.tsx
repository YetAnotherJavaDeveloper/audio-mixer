import React, { createContext, useContext, type ReactNode } from 'react';
import useAudioEngineHook from './engine.hook';

export interface AudioEngineContextValue extends ReturnType<typeof useAudioEngineHook> { }

const AudioEngineContext = createContext<AudioEngineContextValue | null>(null);

export const AudioEngineProvider = ({ children }: { children: ReactNode }) => {
  const audioEngine = useAudioEngineHook();
  return (
    <AudioEngineContext.Provider value={audioEngine}>
      {children}
    </AudioEngineContext.Provider>
  );
};

export const useAudioEngineContext = (): AudioEngineContextValue => {
  const ctx = useContext(AudioEngineContext);
  if (!ctx) {
    throw new Error('useAudioEngine must be used within an AudioEngineProvider');
  }
  return ctx;
};