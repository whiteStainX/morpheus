import React, { useState, useCallback, useMemo } from 'react';
import { Box, Text, useInput, useApp } from 'ink';
import { changeDirectory, listDirectory, readFile, formatPath } from '../utils/virtualFileSystem.js';

interface FileSystemProps {
  logo: string;
}

type LineType = 'system' | 'command' | 'response';

interface TerminalLine {
  type: LineType;
  text: string;
}

const helpLines = [
  'help             Display this help listing',
  'ls [path]        List directory contents',
  'cd <path>        Change the working directory',
  'pwd              Print working directory',
  'cat <file>       View file contents',
  'motd             Read the daily transmission',
  'banner           Redraw system crest',
  'clear            Wipe the screen',
  'exit             Shutdown workstation (aliases: /quit, shutdown)'
];

function FileSystem({ logo }: FileSystemProps): React.ReactElement {
  const { exit } = useApp();
  const [input, setInput] = useState('');
  const [currentPath, setCurrentPath] = useState<string[]>(['home', 'operator']);
  const [log, setLog] = useState<TerminalLine[]>([
    { type: 'system', text: 'MORPHEUS-86 WORKSTATION // DESKTOP CHANNEL ACTIVE' },
    { type: 'system', text: 'AUTH ACCESS REQUIRED :: TYPE `help` FOR COMMANDS' }
  ]);

  const prompt = useMemo(() => `morpheus${formatPath(currentPath)}$`, [currentPath]);

  const appendLines = useCallback((lines: TerminalLine[]) => {
    setLog((previous) => [...previous, ...lines]);
  }, []);

  const shutdown = useCallback(() => {
    appendLines([{ type: 'system', text: 'Disengaging Morpheus interface...' }]);
    setTimeout(() => exit(), 500);
  }, [appendLines, exit]);

  const executeCommand = useCallback(
    (rawInput: string) => {
      const displayedCommand = rawInput;
      const trimmed = rawInput.trim();

      appendLines([{ type: 'command', text: `${prompt} ${displayedCommand}`.trimEnd() }]);

      if (trimmed === '') {
        return;
      }

      const [command, ...args] = trimmed.split(/\s+/);

      switch (command) {
        case 'help': {
          appendLines(helpLines.map((line) => ({ type: 'response', text: line })));
          break;
        }
        case 'ls': {
          const target = args[0];
          const { entries, error } = listDirectory(currentPath, target);
          if (error) {
            appendLines([{ type: 'response', text: error }]);
          } else if (entries) {
            appendLines([{ type: 'response', text: entries.join('  ') }]);
          }
          break;
        }
        case 'pwd': {
          appendLines([{ type: 'response', text: formatPath(currentPath) }]);
          break;
        }
        case 'cd': {
          const target = args[0];
          if (!target) {
            appendLines([{ type: 'response', text: 'cd: target path required' }]);
            break;
          }
          const { newPath, error } = changeDirectory(currentPath, target);
          if (error) {
            appendLines([{ type: 'response', text: error }]);
          } else if (newPath) {
            setCurrentPath(newPath);
            appendLines([{ type: 'system', text: `Directory changed to ${formatPath(newPath)}` }]);
          }
          break;
        }
        case 'cat': {
          const target = args[0];
          if (!target) {
            appendLines([{ type: 'response', text: 'cat: file path required' }]);
            break;
          }
          const { content, error } = readFile(currentPath, target);
          if (error) {
            appendLines([{ type: 'response', text: error }]);
          } else if (content) {
            appendLines(content.split('\n').map((line) => ({ type: 'response', text: line })));
          }
          break;
        }
        case 'motd': {
          const { content, error } = readFile(currentPath, '/etc/motd');
          if (error) {
            appendLines([{ type: 'response', text: error }]);
          } else if (content) {
            appendLines(content.split('\n').map((line) => ({ type: 'response', text: line })));
          }
          break;
        }
        case 'banner': {
          appendLines(logo.split('\n').map((line) => ({ type: 'response', text: line })));
          break;
        }
        case 'clear': {
          setLog([]);
          break;
        }
        case '/quit':
        case 'exit':
        case 'shutdown': {
          shutdown();
          break;
        }
        default: {
          appendLines([{ type: 'response', text: `${command}: command not recognized` }]);
        }
      }
    },
    [appendLines, currentPath, logo, prompt, shutdown]
  );

  useInput((inputChar, key) => {
    if (key.return) {
      const currentInput = input;
      setInput('');
      executeCommand(currentInput);
    } else if (key.backspace || key.delete) {
      setInput((previous) => previous.slice(0, -1));
    } else if (key.ctrl && inputChar === 'c') {
      shutdown();
    } else if (!key.tab && !key.leftArrow && !key.rightArrow && !key.upArrow && !key.downArrow) {
      setInput((previous) => previous + inputChar);
    }
  });

  const colorByType: Record<LineType, string> = {
    system: 'greenBright',
    command: 'gray',
    response: 'green',
  };

  return (
    <Box flexDirection="column" alignItems="center" paddingTop={1}>
      <Text color="greenBright">{logo}</Text>
      <Box
        width={72}
        borderStyle="single"
        borderColor="green"
        paddingX={1}
        paddingY={0}
        marginTop={1}
        flexDirection="column"
      >
        <Text color="green">Morpheus-86 Desktop Shell // Revision 2.3</Text>
        <Text color="green">CRT sync stable · phosphor mode engaged</Text>
      </Box>
      <Box
        width={72}
        flexDirection="column"
        borderStyle="single"
        borderColor="green"
        paddingX={1}
        paddingY={0}
        marginTop={1}
      >
        {log.map((line, index) => (
          <Text key={`${line.type}-${index}`} color={colorByType[line.type]}>
            {line.text}
          </Text>
        ))}
        <Box>
          <Text color="greenBright">{prompt} </Text>
          <Text color="green">{input}</Text>
          <Text color="greenBright">▌</Text>
        </Box>
      </Box>
    </Box>
  );
}

export default FileSystem;
