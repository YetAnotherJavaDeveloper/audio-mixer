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
  const hours = Math.floor(ms / 3600000);
  const minutes = Math.floor((ms % 3600000) / 60000);
  const seconds = Math.floor((ms % 60000) / 1000);
  const millis = ms % 1000;
  return `${hours}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}.${millis.toString().padStart(3, '0')}`;
};

export const audioUtils = {
  extractAudioMetadata,
  formatMilliseconds,
};
