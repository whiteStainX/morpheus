import React, { useState, useEffect, useCallback } from 'react';
import { Box } from 'ink';
import Welcome from './components/Welcome.js';
import FileSystem from './components/FileSystem.js';
import Screen from './components/Screen.js';
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
    <Screen>
      {screen === 'welcome' ? (
        <Welcome logo={welcomeLogoContent} onBootComplete={handleBootComplete} />
      ) : (
        <Box borderStyle="single" borderColor="green" padding={1} flexDirection="column">
          <FileSystem logo={desktopLogoContent} />
        </Box>
      )}
    </Screen>
  );
}

export default App;
