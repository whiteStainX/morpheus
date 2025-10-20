import React, {FC} from 'react';
import {Box, Text} from 'ink';

export type LoadingBarProps = {
  progress: number;
  width?: number;
  label?: string;
};

const clamp = (value: number, min: number, max: number) => Math.min(Math.max(value, min), max);

export const LoadingBar: FC<LoadingBarProps> = ({progress, width = 30, label}) => {
  const normalized = clamp(progress, 0, 1);
  const filled = Math.round(normalized * width);
  const empty = width - filled;

  return (
    <Box flexDirection="column">
      {label ? (
        <Box marginBottom={1}>
          <Text color="cyan">{label}</Text>
        </Box>
      ) : null}
      <Box>
        <Text color="green">{'█'.repeat(filled)}</Text>
        <Text>{'░'.repeat(empty)}</Text>
      </Box>
    </Box>
  );
};

export default LoadingBar;
