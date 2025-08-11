import React, { useCallback } from 'react'
import { Input } from './ui/input';
import { Label } from './ui/label';

export interface FileInputProps {
    onFileSelected: (_file: File) => void;
}

export const AudioFileInput: React.FC<FileInputProps> = ({ onFileSelected }) => {

    const [currentFilename, setCurrentFilename] = React.useState<string | undefined>(undefined);

    const handleFileChange = useCallback(async (e: React.ChangeEvent<HTMLInputElement>) => {
        const file = e.target.files?.[0];
        const filename = file?.name || 'unknown file';
        if (!file || !filename) return;
        setCurrentFilename(filename);

        onFileSelected(file);
    }, [onFileSelected]);

    return (
        <div className="grid w-full max-w-sm items-center gap-3">
            <Label htmlFor="audio-file">Choose an audio file</Label>
            <Input
                id="audio-file"
                type="file"
                accept="audio/*"
                onChange={handleFileChange}
            />
            {currentFilename && (
                <p className="text-sm text-gray-500 mt-2">
                    Selected file: {currentFilename}
                </p>
            )}
        </div>
    )
}

export default AudioFileInput;