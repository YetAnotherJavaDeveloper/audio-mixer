import React, { useCallback, useImperativeHandle, useRef } from 'react'
import { Label } from './ui/label';
import { FileUp } from 'lucide-react';
import { buttonVariants } from './ui/button';
import { cn } from '@/lib/utils';

export interface FileInputProps extends React.ComponentProps<'input'> {
  onFileSelected: (_file: File) => void;
}

export interface FileInputRef {
  getCurrentFilename: () => string | undefined;
  reset: () => void;
}

const AudioFileInput = React.forwardRef<FileInputRef, FileInputProps>(
  ({ onFileSelected, className }, ref) => {

    const inputRef = useRef<HTMLInputElement>(null);
    const [_currentFilename, setCurrentFilename] = React.useState<string | undefined>(undefined);

    useImperativeHandle(ref, () => ({
      getCurrentFilename: () => _currentFilename,
      reset: () => {
        if (inputRef.current) {
          inputRef.current.value = '';
        }
      }
    }));

    const handleFileChange = useCallback(async (e: React.ChangeEvent<HTMLInputElement>) => {
      const file = e.target.files?.[0];
      const filename = file?.name || 'unknown file';
      if (!file || !filename) return;
      setCurrentFilename(filename);

      onFileSelected(file);
    }, [onFileSelected]);

    const classname = cn(
      buttonVariants({ variant: 'outline', size: 'sm' }),
      className,
      "cursor-pointer",
      "flex items-center",
    );

    return (
      <div className={classname}>
        <Label htmlFor="file-upload">
          <FileUp />
        </Label>
        <input ref={inputRef} className="sr-only" id="file-upload" type="file" accept="audio/*" onChange={handleFileChange} />
      </div>
    )
  }
);

export default AudioFileInput;