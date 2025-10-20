import React, { useState, useEffect } from 'react';
import { Box, Text } from 'ink';
import StatusBar from './StatusBar.js';
import { bootMessages } from '../data/bootMessages.js';

interface WelcomeProps {
  logo: string;
  onBootComplete: () => void;
}

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
        onBootComplete();
      }
    }, 1000);

    return () => clearInterval(interval);
  }, [onBootComplete]);

  return (
    <Box flexDirection="column" alignItems="center" paddingTop={1}>
      <Text color="greenBright">{logo}</Text>
      <Box
        width={72}
        borderStyle="single"
        borderColor="green"
        paddingX={1}
        paddingY={0}
        marginTop={1}
        flexDirection="column"
      >
        <Text color="green">Morpheus-86 Bootloader // ROM v2.3</Text>
        <Text color="green">Warming cathode emitters · Aligning scanlines</Text>
      </Box>
      <Box
        width={72}
        borderStyle="single"
        borderColor="green"
        paddingX={1}
        paddingY={0}
        marginTop={1}
      >
        <StatusBar message={status} progress={progress} />
      </Box>
    </Box>
  );
}

export default Welcome;
