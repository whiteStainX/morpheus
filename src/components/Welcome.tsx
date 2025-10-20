import React, { useState, useEffect } from 'react';
import { Box, Text } from 'ink';
import StatusBar from './StatusBar.js';

interface WelcomeProps {
  logo: string;
  onBootComplete: () => void;
}

import { bootMessages } from '../data/bootMessages.js';

function Welcome({ logo, onBootComplete }: WelcomeProps): React.ReactElement {
  const [status, setStatus] = useState(bootMessages[0].text);
  const [progress, setProgress] = useState(bootMessages[0].progress);

  useEffect(() => {
    let messageIndex = 0;

    const interval = setInterval(() => {
      if (messageIndex < bootMessages.length) {
        const currentMessage = bootMessages[messageIndex];
        setStatus(currentMessage.text);
        setProgress(currentMessage.progress);
        messageIndex++;
      } else {
        clearInterval(interval);
        onBootComplete(); // Call when boot is complete
      }
    }, 1000); // Display each message for 1 second

    return () => clearInterval(interval);
  }, [onBootComplete]);
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
