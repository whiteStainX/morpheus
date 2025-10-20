export type CommandResponse = {
  lines: string[];
  clear?: boolean;
};

const helpMessage = [
  'Available commands:',
  '  help   - show this help text',
  '  record - guidance for recording your screen session',
  '  status - display the current mission status',
  '  clear  - wipe the command output window',
  '  exit   - hint about leaving Morpheus',
];

const recordMessage = [
  'Screen capture prep checklist:',
  '  • Ensure your preferred screen recorder is running.',
  '  • Trigger the Morpheus CLI actions you want captured.',
  '  • When finished, stop the recorder to save your video.',
  '  • Optional: run `status` to log the session outcome.',
];

const statusMessage = [
  'Morpheus subsystems nominal.',
  `Timestamp: ${new Date().toLocaleString()}`,
  'Ready for operator input.',
];

export const executeCommand = async (rawCommand: string): Promise<CommandResponse> => {
  const command = rawCommand.trim().toLowerCase();

  if (!command) {
    return {lines: []};
  }

  switch (command) {
    case 'help':
      return {lines: helpMessage};
    case 'record':
      return {lines: recordMessage};
    case 'status':
      return {lines: statusMessage};
    case 'clear':
      return {lines: [], clear: true};
    case 'exit':
      return {
        lines: [
          'To exit Morpheus press Ctrl+C. Your session transcript is preserved above.',
        ],
      };
    default:
      return {
        lines: [
          `Unrecognised command: "${rawCommand.trim()}"`,
          'Type `help` to see the list of supported commands.',
        ],
      };
  }
};
