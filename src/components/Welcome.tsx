import React, { useState, useEffect } from 'react';
import { Box, Text } from 'ink';
import StatusBar from './StatusBar.js';
import { bootMessages } from '../data/bootMessages.js';

interface WelcomeProps {
  logo: string;
  onBootComplete: () => void;
}

const BlinkingCursor = () => {
  const [visible, setVisible] = useState(true);
  useEffect(() => {
    const timer = setInterval(() => {
      setVisible((v) => !v);
    }, 500);
    return () => clearInterval(timer);
  }, []);

  return <Text color="green">{visible ? '█' : ' '}</Text>;
};

function Welcome({ logo, onBootComplete }: WelcomeProps): React.ReactElement {
  const [messageIndex, setMessageIndex] = useState(0);
  const [typedMessage, setTypedMessage] = useState('');
  const [progress, setProgress] = useState(0);

  useEffect(() => {
    if (messageIndex >= bootMessages.length) {
      setTimeout(onBootComplete, 500);
      return;
    }

    const currentMessage = bootMessages[messageIndex];
    setProgress(currentMessage.progress);

    if (typedMessage.length < currentMessage.text.length) {
      const typingTimer = setTimeout(() => {
        setTypedMessage(currentMessage.text.slice(0, typedMessage.length + 1));
      }, 50);
      return () => clearTimeout(typingTimer);
    } else {
      const messageTimer = setTimeout(() => {
        setMessageIndex(messageIndex + 1);
        setTypedMessage('');
      }, 1000);
      return () => clearTimeout(messageTimer);
    }
  }, [messageIndex, typedMessage, onBootComplete]);

  return (
    <Box flexDirection="column" alignItems="center" paddingTop={2}>
      <Text color="green">{logo}</Text>
      <Box marginTop={1} minHeight={1}>
        {messageIndex < bootMessages.length && (
          <Box>
            <Text color="green">{typedMessage}</Text>
            <BlinkingCursor />
          </Box>
        )}
      </Box>
      <Box marginTop={1}>
        <StatusBar
          message={bootMessages[messageIndex]?.text || 'Boot sequence complete.'}
          progress={progress}
        />
      </Box>
    </Box>
  );
}

export default Welcome;
