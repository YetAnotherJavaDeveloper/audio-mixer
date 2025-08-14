import React, { useCallback, useImperativeHandle, useRef } from 'react'
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
          setCurrentFilename(undefined);
        }
      }
    }));

    const handleFileChange = useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
      const file = e.target.files?.[0];
      if (!file) return;
      setCurrentFilename(file.name);
      onFileSelected(file);
    }, [onFileSelected]);

    const handleClick = useCallback(() => {
      inputRef.current?.click();
    }, []);

    const classname = cn(
      buttonVariants({ variant: 'outline', size: 'sm' }),
      className,
      "cursor-pointer flex items-center"
    );

    return (
      <>
        <div className={classname} onClick={handleClick}>
          <FileUp className="mr-2" />
          {_currentFilename ?? "Choose file"}
        </div>
        <input
          ref={inputRef}
          type="file"
          accept="audio/*"
          className="hidden"
          onChange={handleFileChange}
        />
      </>
    )
  }
);

export default AudioFileInput;
