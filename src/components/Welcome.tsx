import React, { useState, useEffect } from 'react';
import { Box, Text } from 'ink';

interface WelcomeProps {
  logo: string;
}

function Welcome({ logo }: WelcomeProps): JSX.Element {
  const [status, setStatus] = useState('Initializing system...');

  useEffect(() => {
    const messages = [
      'Loading modules...',
      'Establishing secure connection...',
      'Preparing user interface...',
      'System ready.',
    ];
    let i = 0;
    const interval = setInterval(() => {
      if (i < messages.length) {
        setStatus(messages[i]);
        i++;
      } else {
        clearInterval(interval);
      }
    }, 1000);

    return () => clearInterval(interval);
  }, []);

  return (
    <Box flexDirection="column" alignItems="center" paddingTop={2}>
      <Text color="cyan">{logo}</Text>
      <Box marginTop={1}>
        <Text color="green">Status: {status}</Text>
      </Box>
    </Box>
  );
}

export default Welcome;
