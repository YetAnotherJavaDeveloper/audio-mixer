import { round } from 'radashi';

const extractAudioMetadata = async (file: File) => {
  const audio = new Audio(URL.createObjectURL(file));
  const promise = new Promise((resolve) => {
    audio.onloadedmetadata = () => {
      const metadata: Record<string, number> = {};
      metadata.duration = audio.duration;
      resolve(metadata);
    };
    audio.onerror = () => {
      console.error('Error loading audio metadata');
      resolve({});
    };
  });

  return (await promise) as { duration: number };
};

const formatMilliseconds = (ms: number) => {
  const hours = round(ms / (3600 * 1000));
  const minutes = round((ms % (3600 * 1000)) / (60 * 1000));
  const seconds = round((ms % (60 * 1000)) / 1000);
  const millis = round(ms % 1000);
  // format as HH:mm:ss.SSS
  return `${String(hours).padStart(2, '0')}:${String(minutes).padStart(2, '0')}:${String(seconds).padStart(2, '0')}.${String(millis).padStart(3, '0')}`;
};

export const audioUtils = {
  extractAudioMetadata,
  formatMilliseconds,
};
