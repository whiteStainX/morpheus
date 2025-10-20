import React, { useState, useEffect, useCallback } from 'react';
import { Box, Text } from 'ink'; // Import Box and Text here
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
    <Box borderStyle="single" borderColor="green" flexDirection="column" padding={1}>
      {screen === 'welcome' ? (
        <Welcome logo={welcomeLogoContent} onBootComplete={handleBootComplete} />
      ) : (
        <FileSystem logo={desktopLogoContent} />
      )}
    </Box>
  );
}

export default App;
