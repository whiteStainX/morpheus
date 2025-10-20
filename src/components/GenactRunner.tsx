import React, {FC, useEffect, useRef, useState} from 'react';
import {Box, Text, useInput} from 'ink';
import {spawn, ChildProcessWithoutNullStreams} from 'child_process';

export type GenactRunnerProps = {
  duration?: number;
  onComplete: () => void;
};

const DEFAULT_DURATION = 10_000;

const splitLines = (chunk: Buffer): string[] =>
  chunk
    .toString()
    .replace(/\r/g, '')
    .split('\n')
    .filter(line => line.length > 0);

export const GenactRunner: FC<GenactRunnerProps> = ({
  duration = DEFAULT_DURATION,
  onComplete,
}) => {
  const [output, setOutput] = useState<string[]>([]);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);
  const processRef = useRef<ChildProcessWithoutNullStreams | null>(null);
  const completedRef = useRef(false);

  const finish = () => {
    if (completedRef.current) {
      return;
    }

    completedRef.current = true;
    onComplete();
  };

  useInput(() => {
    const child = processRef.current;
    if (child && !child.killed) {
      child.kill('SIGINT');
    }
    finish();
  });

  useEffect(() => {
    try {
      const child = spawn('genact');
      processRef.current = child;

      const timer: ReturnType<typeof setTimeout> = setTimeout(() => {
        if (!child.killed) {
          child.kill('SIGINT');
        }
      }, duration);

      child.stdout.on('data', chunk => {
        setOutput(prev => [...prev, ...splitLines(chunk)]);
      });

      child.stderr.on('data', chunk => {
        setErrorMessage(chunk.toString());
      });

      child.on('error', error => {
        setErrorMessage(error.message);
        finish();
      });

      child.on('close', () => {
        clearTimeout(timer);
        finish();
      });

      return () => {
        clearTimeout(timer);
        if (!child.killed) {
          child.kill('SIGINT');
        }
      };
    } catch (error) {
      setErrorMessage(error instanceof Error ? error.message : 'Failed to execute genact.');
      finish();
      return undefined;
    }
  }, [duration, onComplete]);

  return (
    <Box flexDirection="column" paddingX={1} paddingY={1}>
      <Box marginBottom={1}>
        <Text color="cyan">Simulating busy terminal &mdash; press any key to continue</Text>
      </Box>
      {errorMessage ? (
        <Text color="red">{errorMessage}</Text>
      ) : output.length > 0 ? (
        output.map((line, index) => (
          <Text key={`${index}-${line}`}>{line}</Text>
        ))
      ) : (
        <Text color="gray">Waiting for genact output...</Text>
      )}
    </Box>
  );
};

export default GenactRunner;
