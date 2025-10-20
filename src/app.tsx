import React, { useState, useEffect } from 'react';
import Welcome from './components/Welcome';
import { readTextFile } from './utils/fileReader';

function App() {
  const [logoContent, setLogoContent] = useState('');

  useEffect(() => {
    const content = readTextFile('src/assets/logo.txt');
    setLogoContent(content);
  }, []);

  return <Welcome logo={logoContent} />;
}

export default App;
