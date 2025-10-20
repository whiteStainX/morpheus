import React, { useState, useEffect, useCallback } from 'react';
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

  if (screen === 'welcome') {
    return <Welcome logo={welcomeLogoContent} onBootComplete={handleBootComplete} />;
  } else {
    return <FileSystem logo={desktopLogoContent} />;
  }
}

export default App;
