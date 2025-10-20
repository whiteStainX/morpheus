import React from 'react';
import { Box, Text } from 'ink';

interface StatusBarProps {
  message: string;
  progress: number; // 0 to 100
}

function StatusBar({ message, progress }: StatusBarProps): JSX.Element {
  const barLength = 20; // Length of the progress bar
  const filledBlocks = Math.floor((progress / 100) * barLength);
  const emptyBlocks = barLength - filledBlocks;

  const progressBar = '█'.repeat(filledBlocks) + '░'.repeat(emptyBlocks);

  return (
    <Box flexDirection="column">
      <Text color="green">{message} ({progress}%)</Text>
      <Text color="green">{progressBar}</Text>
    </Box>
  );
}

export default StatusBar;
