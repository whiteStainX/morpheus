import React from 'react';
import { Box, Text } from 'ink';

interface StatusBarProps {
  message: string;
  progress: number;
}

function StatusBar({ message, progress }: StatusBarProps): JSX.Element {
  const barLength = 30;
  const clampedProgress = Math.max(0, Math.min(progress, 100));
  const filledBlocks = Math.round((clampedProgress / 100) * barLength);
  const emptyBlocks = barLength - filledBlocks;

  const progressBar = '▓'.repeat(filledBlocks) + '░'.repeat(emptyBlocks);
  const progressLabel = `${clampedProgress.toString().padStart(3, ' ')}%`;

  return (
    <Box flexDirection="column" alignItems="stretch">
      <Text color="greenBright">{message}</Text>
      <Text color="green">[{progressBar}] {progressLabel}</Text>
    </Box>
  );
}

export default StatusBar;
