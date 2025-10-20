import React, { useState, useEffect } from 'react';
import { Box, Text, useInput, useApp } from 'ink';
import useStdoutDimensions from 'ink-use-stdout-dimensions';
import { readTextFile } from '../utils/fileReader.js';

interface File {
  type: 'file';
  content: string;
}

interface Directory {
  type: 'directory';
  children: { [key: string]: File | Directory };
}

type FileSystemNode = File | Directory;

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
  const { columns } = useStdoutDimensions();
  const [input, setInput] = useState('');
  const [fileSystem, setFileSystem] = useState<Directory | null>(null);
  const [currentPath, setCurrentPath] = useState('/');
  const [output, setOutput] = useState<Array<{ type: 'command' | 'response'; text: string }>>([
    { type: 'response', text: 'Welcome to Morpheus OS.' },
    { type: 'response', text: "Type 'help' for a list of commands." },
  ]);

  useEffect(() => {
    const fsData = readTextFile('src/data/filesystem.json');
    if (fsData) {
      setFileSystem(JSON.parse(fsData));
    }
  }, []);

  const instructions = [
    `List of commands:`,
    `  'help' - display this list of commands`,
    `  'ls'   - list directory contents`,
    `  'cd <dir>' - change directory`,
    `  'cat <file>' - view file contents`,
    `  '/quit' - exit the OS`,
  ];

  const getNodeByPath = (path: string): FileSystemNode | null => {
    if (!fileSystem) return null;
    const pathParts = path.split('/').filter(p => p);
    let currentNode: FileSystemNode = fileSystem['/'];
    for (const part of pathParts) {
      if (currentNode.type === 'directory' && currentNode.children[part]) {
        currentNode = currentNode.children[part];
      } else {
        return null;
      }
    }
    return currentNode;
  };

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
      } else if (command === 'ls') {
        const currentDir = getNodeByPath(currentPath);
        if (currentDir?.type === 'directory') {
          const content = Object.keys(currentDir.children).join('  ');
          newOutput.push({ type: 'response', text: content });
        }
        setOutput(newOutput);
      } else if (command.startsWith('cd ')) {
        const targetDir = command.split(' ')[1];
        if (targetDir) {
          let newPath;
          if (targetDir === '..') {
            if (currentPath === '/') {
              newPath = '/';
            } else {
              const pathParts = currentPath.split('/').filter(p => p);
              pathParts.pop();
              newPath = '/' + pathParts.join('/');
            }
          } else if (targetDir.startsWith('/')) {
            newPath = targetDir;
          } else {
            newPath = currentPath === '/' ? `/${targetDir}` : `${currentPath}/${targetDir}`;
          }

          if (newPath.length > 1 && newPath.endsWith('/')) {
            newPath = newPath.slice(0, -1);
          }

          const targetNode = getNodeByPath(newPath);
          if (targetNode?.type === 'directory') {
            setCurrentPath(newPath);
          } else {
            newOutput.push({ type: 'response', text: `cd: ${targetDir}: No such file or directory` });
          }
        }
        setOutput(newOutput);
      } else if (command.startsWith('cat ')) {
        const targetFile = command.split(' ')[1];
        if (targetFile) {
          const filePath = targetFile.startsWith('/') ? targetFile : (currentPath === '/' ? `/${targetFile}`: `${currentPath}/${targetFile}`);
          const targetNode = getNodeByPath(filePath);
          if (targetNode?.type === 'file') {
            newOutput.push({ type: 'response', text: targetNode.content });
          } else {
            newOutput.push({ type: 'response', text: `cat: ${targetFile}: No such file or directory` });
          }
        }
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
        width={columns}
      >
        {output.map((line, index) => (
          <Text key={index} color={line.type === 'command' ? 'gray' : 'green'}>
            {line.text}
          </Text>
        ))}
        <Box>
          <Text color="green">morpheus:{currentPath}$ </Text>
          <Text color="green">{input}</Text>
          <BlinkingCursor />
        </Box>
      </Box>
    </Box>
  );
}

export default FileSystem;
