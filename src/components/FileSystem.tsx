import React, { useState, useEffect } from 'react';
import { Box, Text, useInput, useApp } from 'ink';

interface FileSystemProps {
  logo: string;
}

function FileSystem({ logo }: FileSystemProps): React.ReactElement {
  const { exit } = useApp();
  const [input, setInput] = useState('');
  const [output, setOutput] = useState<Array<{ type: 'command' | 'response'; text: string }>>([]);

  const instructions = [
    `Welcome to Morpheus OS.`,
    `Type 'help' for a list of commands.`,
    `Use 'ls' to list directory contents.`,
    `Use 'cd <dir>' to change directory.`,
    `Use 'cat <file>' to view file contents.`,
    `Type '/quit' to exit the OS.`,
  ];

  useInput((inputChar, key) => {
    if (key.return) {
      // On Enter key
      const command = input.trim();
      setOutput((prevOutput) => [...prevOutput, { type: 'command', text: `morpheus:/$ ${command}` }]);
      setInput('');

      if (command === '/quit') {
        setOutput((prevOutput) => [...prevOutput, { type: 'response', text: 'Shutting down Morpheus OS...' }]);
        setTimeout(() => exit(), 500); // Exit after a short delay
      } else if (command === 'help') {
        setOutput((prevOutput) => [...prevOutput, { type: 'response', text: instructions.join('\n') }]);
      } else if (command === '') {
        // Do nothing for empty command
      } else {
        setOutput((prevOutput) => [...prevOutput, { type: 'response', text: `Command not found: ${command}` }]);
      }
    } else if (key.backspace || key.delete) {
      // On Backspace/Delete key
      setInput((prevInput) => prevInput.slice(0, -1));
    } else if (!key.tab && !key.leftArrow && !key.rightArrow && !key.upArrow && !key.downArrow) {
      // Append character to input
      setInput((prevInput) => prevInput + inputChar);
    }
  });

  return (
    <Box flexDirection="column" alignItems="center" paddingTop={1}>
      <Text color="green">{logo}</Text>
      <Box marginTop={1} marginBottom={1} borderStyle="single" borderColor="green" paddingX={1} flexDirection="column">
        {instructions.map((line, index) => (
          <Box key={index}>
            <Text color="green">{line}</Text>
          </Box>
        ))}
      </Box>
      <Box width="100%" height={10} flexDirection="column" borderStyle="single" borderColor="green" paddingX={1} paddingY={0}>
        {output.map((line, index) => (
          <Text key={index} color={line.type === 'command' ? 'gray' : 'green'}>{line.text}</Text>
        ))}
        <Box>
          <Text color="green">morpheus:/$ </Text>
          <Text color="green">{input}</Text>
        </Box>
      </Box>
    </Box>
  );
}

export default FileSystem;
