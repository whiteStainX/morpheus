import React, { useState, useEffect } from 'react';
import { Box, Text, useInput, useApp } from 'ink';

interface FileSystemProps {
  logo: string;
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

function FileSystem({ logo }: FileSystemProps): React.ReactElement {
  const { exit } = useApp();
  const [input, setInput] = useState('');
  const [output, setOutput] = useState<Array<{ type: 'command' | 'response'; text: string }>>([
    { type: 'response', text: 'Welcome to Morpheus OS.' },
    { type: 'response', text: "Type 'help' for a list of commands." },
  ]);

  const instructions = [
    `List of commands:`,
    `  'help' - display this list of commands`,
    `  'ls'   - list directory contents`,
    `  'cd <dir>' - change directory`,
    `  'cat <file>' - view file contents`,
    `  '/quit' - exit the OS`,
  ];

  useInput((inputChar, key) => {
    if (key.return) {
      const command = input.trim();
      const newOutput = [...output, { type: 'command' as const, text: `morpheus:/$ ${command}` }];

      if (command === '/quit') {
        newOutput.push({ type: 'response', text: 'Shutting down Morpheus OS...' });
        setOutput(newOutput);
        setTimeout(() => exit(), 500);
      } else if (command === 'help') {
        newOutput.push({ type: 'response', text: instructions.join('\n') });
        setOutput(newOutput);
      } else if (command !== '') {
        newOutput.push({ type: 'response', text: `Command not found: ${command}` });
        setOutput(newOutput);
      } else {
        setOutput(newOutput);
      }
      setInput('');
    } else if (key.backspace || key.delete) {
      setInput((prevInput) => prevInput.slice(0, -1));
    } else if (!key.ctrl && !key.meta && inputChar) {
      setInput((prevInput) => prevInput + inputChar);
    }
  });

  return (
    <Box flexDirection="column" alignItems="stretch" width="100%">
      <Box justifyContent="center">
        <Text color="green">{logo}</Text>
      </Box>
      <Box
        flexDirection="column"
        borderStyle="single"
        borderColor="green"
        paddingX={1}
        marginTop={1}
        height={15}
      >
        {output.map((line, index) => (
          <Text key={index} color={line.type === 'command' ? 'gray' : 'green'}>
            {line.text}
          </Text>
        ))}
        <Box>
          <Text color="green">morpheus:/$ </Text>
          <Text color="green">{input}</Text>
          <BlinkingCursor />
        </Box>
      </Box>
    </Box>
  );
}

export default FileSystem;
