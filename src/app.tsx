import React, { useState, useEffect, useCallback } from 'react';
import { Box, Text } from 'ink';
import Welcome from './components/Welcome.js';
import FileSystem from './components/FileSystem.js';
import { readTextFile } from './utils/fileReader.js';

function App() {
  const [welcomeLogoContent, setWelcomeLogoContent] = useState('');
  const [desktopLogoContent, setDesktopLogoContent] = useState('');
  const [screen, setScreen] = useState<'welcome' | 'fileSystem'>('welcome');

  useEffect(() => {
    setWelcomeLogoContent(readTextFile('src/assets/logo.txt'));
    setDesktopLogoContent(readTextFile('src/assets/logo_desktop.txt'));
  }, []);

  const handleBootComplete = useCallback(() => {
    setScreen('fileSystem');
  }, []);

  return (
    <Box
      flexDirection="column"
      alignItems="center"
      justifyContent="center"
      padding={1}
      flexGrow={1}
      backgroundColor="black"
    >
      <Box
        flexDirection="column"
        alignItems="center"
        borderStyle="double"
        borderColor="green"
        paddingX={2}
        paddingY={1}
        width={80}
      >
        <Text color="green">MORPHEUS-86 INTERFACE // CRT MODE</Text>
        {screen === 'welcome' ? (
          <Welcome logo={welcomeLogoContent} onBootComplete={handleBootComplete} />
        ) : (
          <FileSystem logo={desktopLogoContent} />
        )}
      </Box>
    </Box>
  );
}

export default App;
