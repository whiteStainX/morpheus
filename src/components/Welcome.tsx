import React, { useState, useEffect } from 'react';
import { Box, Text } from 'ink';
import StatusBar from './StatusBar.js';

interface WelcomeProps {
  logo: string;
}

import { bootMessages } from '../data/bootMessages.js';

function Welcome({ logo }: WelcomeProps): React.ReactElement {
  const [status, setStatus] = useState(bootMessages[0]);
  const [progress, setProgress] = useState(0);

  useEffect(() => {
    const messages = bootMessages;
    let messageIndex = 0;
    let currentProgress = 0;
    const totalSteps = messages.length * 2; // Each message and a progress increment
    const progressPerStep = 100 / totalSteps;

    const interval = setInterval(() => {
      if (messageIndex < messages.length) {
        setStatus(messages[messageIndex]);
        currentProgress += progressPerStep;
        setProgress(Math.min(100, Math.floor(currentProgress)));
        messageIndex++;
      } else if (currentProgress < 100) {
        currentProgress += progressPerStep;
        setProgress(Math.min(100, Math.floor(currentProgress)));
      } else {
        clearInterval(interval);
      }
    }, 500);

    return () => clearInterval(interval);
  }, []);
  return (
    <Box flexDirection="column" alignItems="center" paddingTop={2}>
      <Text color="cyan">{logo}</Text>
      <Box marginTop={1}>
        <StatusBar message={status} progress={progress} />
      </Box>
    </Box>
  );
}

export default Welcome;
