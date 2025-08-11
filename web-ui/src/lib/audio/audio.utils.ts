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

export const audioUtils = {
  extractAudioMetadata,
};
