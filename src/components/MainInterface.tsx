import React, {FC, useState} from 'react';
import {Box, Text} from 'ink';
import TextInput from 'ink-text-input';
import {executeCommand} from '../utils/commandExecutor.js';

export type MainInterfaceProps = {
  logo: string;
};

export const MainInterface: FC<MainInterfaceProps> = ({logo}) => {
  const [inputValue, setInputValue] = useState('');
  const [history, setHistory] = useState<string[]>([]);
  const [isRunningCommand, setIsRunningCommand] = useState(false);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);

  const handleSubmit = async () => {
    const command = inputValue;
    setInputValue('');
    setErrorMessage(null);

    if (!command.trim()) {
      return;
    }

    setIsRunningCommand(true);

    try {
      const response = await executeCommand(command);
      setHistory(previous => {
        const baseHistory = response.clear ? [] : previous;
        return [...baseHistory, `> ${command}`, ...response.lines];
      });
    } catch (error) {
      const message = error instanceof Error ? error.message : 'Unknown command error';
      setHistory(previous => [...previous, `> ${command}`, message]);
      setErrorMessage(message);
    } finally {
      setIsRunningCommand(false);
    }
  };

  return (
    <Box flexDirection="column" padding={1} gap={1}>
      <Box borderStyle="round" borderColor="gray" paddingX={2} paddingY={1}>
        <Text color="cyan">{logo}</Text>
      </Box>

      <Box flexDirection="column" borderStyle="round" borderColor="gray" padding={1}>
        <Text color="white">Session Output</Text>
        <Box flexDirection="column" marginTop={1}>
          {history.length === 0 ? (
            <Text color="gray">Awaiting your first command. Type `help` to get started.</Text>
          ) : (
            history.map((line, index) => (
              <Text key={`${index}-${line}`}>{line}</Text>
            ))
          )}
        </Box>
      </Box>

      <Box flexDirection="column" borderStyle="round" borderColor="gray" padding={1}>
        <Text color="white">Command Entry</Text>
        <Box marginTop={1}>
          <Text color="green">❯ </Text>
          <TextInput
            value={inputValue}
            onChange={setInputValue}
            onSubmit={handleSubmit}
            placeholder="Type a command and press enter"
            focus={!isRunningCommand}
          />
        </Box>
        {isRunningCommand ? (
          <Box marginTop={1}>
            <Text color="yellow">Processing...</Text>
          </Box>
        ) : null}
        {errorMessage ? (
          <Box marginTop={1}>
            <Text color="red">{errorMessage}</Text>
          </Box>
        ) : null}
      </Box>
    </Box>
  );
};

export default MainInterface;
