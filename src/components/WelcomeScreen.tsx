import React, {FC, useEffect, useRef, useState} from 'react';
import {Box, Text} from 'ink';
import LoadingBar from './LoadingBar.js';

export type WelcomeScreenProps = {
  logo: string;
  loadingDuration?: number;
  onComplete: () => void;
};

const DEFAULT_DURATION = 3_000;

export const WelcomeScreen: FC<WelcomeScreenProps> = ({
  logo,
  loadingDuration = DEFAULT_DURATION,
  onComplete,
}) => {
  const [progress, setProgress] = useState(0);
  const completeRef = useRef(false);
  const timeoutRef = useRef<ReturnType<typeof setTimeout> | undefined>(undefined);

  useEffect(() => {
    const startedAt = Date.now();

    const interval = setInterval(() => {
      const ratio = Math.min((Date.now() - startedAt) / loadingDuration, 1);
      setProgress(ratio);

      if (ratio >= 1 && !completeRef.current) {
        completeRef.current = true;
        clearInterval(interval);
        timeoutRef.current = setTimeout(onComplete, 600);
      }
    }, 100);

    return () => {
      clearInterval(interval);
      if (timeoutRef.current) {
        clearTimeout(timeoutRef.current);
      }
    };
  }, [loadingDuration, onComplete]);

  return (
    <Box flexDirection="column" paddingX={2} paddingY={1}>
      <Box marginBottom={1}>
        <Text color="magenta">{logo}</Text>
      </Box>
      <Text>Booting Morpheus environment...</Text>
      <LoadingBar progress={progress} label="Initialising subsystems" />
    </Box>
  );
};

export default WelcomeScreen;
