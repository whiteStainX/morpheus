import React from 'react';
import { Box, Text } from 'ink';

interface FileSystemProps {
  logo: string;
}

function FileSystem({ logo }: FileSystemProps): React.ReactElement {
  const instructions = [
    `Welcome to Morpheus OS.`,
    `Type 'help' for a list of commands.`,
    `Use 'ls' to list directory contents.`,
    `Use 'cd <dir>' to change directory.`,
    `Use 'cat <file>' to view file contents.`,
  ];

  return (
    <Box flexDirection="column" alignItems="center" paddingTop={1}>
      <Text color="cyan">{logo}</Text>
      <Box marginTop={1} marginBottom={1} borderStyle="single" borderColor="white" paddingX={1}>
        {instructions.map((line, index) => (
          <Text key={index} color="white">{line}</Text>
        ))}
      </Box>
      <Box width="100%" borderStyle="single" borderColor="white" paddingX={1} paddingY={0}>
        <Text color="white">morpheus:/$ </Text>
        {/* CLI input will go here */}
      </Box>
    </Box>
  );
}

export default FileSystem;
