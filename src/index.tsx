#!/usr/bin/env node
import React, {useEffect, useState} from 'react';
import {render, Text} from 'ink';
import path from 'node:path';
import {fileURLToPath} from 'node:url';
import {readFile} from 'node:fs/promises';
import WelcomeScreen from './components/WelcomeScreen.js';
import GenactRunner from './components/GenactRunner.js';
import MainInterface from './components/MainInterface.js';

const defaultLogo = 'morpheus';

const resolveLogoPath = () => {
  const currentPath = fileURLToPath(import.meta.url);
  const directory = path.dirname(currentPath);
  return path.resolve(directory, '../assets/morpheus_logo.txt');
};

const App = () => {
  const [logo, setLogo] = useState(defaultLogo);
  const [phase, setPhase] = useState<'welcome' | 'genact' | 'main'>('welcome');
  const [assetError, setAssetError] = useState<string | null>(null);

  useEffect(() => {
    const loadLogo = async () => {
      try {
        const filePath = resolveLogoPath();
        const contents = await readFile(filePath, 'utf8');
        setLogo(contents);
      } catch (error) {
        setAssetError(
          error instanceof Error
            ? error.message
            : 'Failed to load morpheus logo asset.'
        );
      }
    };

    loadLogo();
  }, []);

  if (assetError) {
    return <Text color="red">{assetError}</Text>;
  }

  switch (phase) {
    case 'welcome':
      return <WelcomeScreen logo={logo} onComplete={() => setPhase('genact')} />;
    case 'genact':
      return <GenactRunner onComplete={() => setPhase('main')} />;
    case 'main':
      return <MainInterface logo={logo} />;
    default:
      return <Text>Unknown phase.</Text>;
  }
};

render(<App />);

export default App;
