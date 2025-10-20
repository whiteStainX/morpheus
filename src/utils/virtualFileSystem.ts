export type FileNode = DirectoryNode | FileLeaf;

export interface DirectoryNode {
  type: 'directory';
  name: string;
  children: FileNode[];
}

export interface FileLeaf {
  type: 'file';
  name: string;
  content: string;
}

const fileSystem: DirectoryNode = {
  type: 'directory',
  name: '/',
  children: [
    {
      type: 'directory',
      name: 'home',
      children: [
        {
          type: 'directory',
          name: 'operator',
          children: [
            {
              type: 'file',
              name: 'readme.txt',
              content: [
                'Morpheus-86 Workstation Notes',
                '--------------------------------',
                'This virtual environment simulates a late 80s terminal.',
                'Type `help` to discover available commands.',
                'Remember: curiosity keeps the system awake.'
              ].join('\n'),
            },
            {
              type: 'file',
              name: 'mission.log',
              content: [
                '██ MORPHEUS OPS LOG ██',
                '12:00 CALIBRATION COMPLETE',
                '12:14 RECEIVED SIGNAL FROM SLEEPER NODE',
                '12:17 SIGNAL LOST — ANALYSIS QUEUED',
                '12:25 Awaiting manual review...'
              ].join('\n'),
            },
          ],
        },
      ],
    },
    {
      type: 'directory',
      name: 'system',
      children: [
        {
          type: 'file',
          name: 'bios.cfg',
          content: [
            '# Morpheus BIOS Configuration',
            'clock_speed = 12MHz',
            'memory_bank = 640KB',
            'ui_mode = phosphor-green',
            'sound = muted'
          ].join('\n'),
        },
        {
          type: 'directory',
          name: 'devices',
          children: [
            {
              type: 'file',
              name: 'tty0.log',
              content: [
                '[BOOT] Terminal line 0 engaged.',
                '[BOOT] CRT scanlines calibrated.',
                '[BOOT] Audio feedback suppressed.'
              ].join('\n'),
            },
          ],
        },
      ],
    },
    {
      type: 'directory',
      name: 'etc',
      children: [
        {
          type: 'file',
          name: 'motd',
          content: [
            'Wake up, operator.',
            'Maintain cover. Observe anomalies.',
            'Signal strength: nominal.'
          ].join('\n'),
        },
      ],
    },
  ],
};

function findNode(path: string[]): FileNode | undefined {
  let current: FileNode = fileSystem;

  for (const segment of path) {
    if (current.type !== 'directory') {
      return undefined;
    }

    const next = current.children.find((child) => child.name === segment);
    if (!next) {
      return undefined;
    }
    current = next;
  }

  return current;
}

function normalizeSegments(targetPath: string, currentPath: string[]): string[] {
  if (targetPath.trim() === '' || targetPath === '.') {
    return [...currentPath];
  }

  if (targetPath === '/') {
    return [];
  }

  const segments = targetPath.split('/').filter(Boolean);
  const base = targetPath.startsWith('/') ? [] : [...currentPath];

  for (const segment of segments) {
    if (segment === '.') {
      continue;
    }
    if (segment === '..') {
      if (base.length > 0) {
        base.pop();
      }
      continue;
    }
    base.push(segment);
  }

  return base;
}

export function changeDirectory(currentPath: string[], targetPath: string): { newPath?: string[]; error?: string } {
  const normalized = normalizeSegments(targetPath, currentPath);
  const node = findNode(normalized);

  if (!node) {
    return { error: `cd: no such file or directory: ${targetPath}` };
  }

  if (node.type !== 'directory') {
    return { error: `cd: not a directory: ${targetPath}` };
  }

  return { newPath: normalized };
}

export function listDirectory(path: string[], targetPath?: string): { entries?: string[]; error?: string } {
  const normalized = targetPath ? normalizeSegments(targetPath, path) : path;
  const node = findNode(normalized);

  if (!node) {
    return { error: 'ls: no such file or directory' };
  }

  if (node.type !== 'directory') {
    return { error: 'ls: not a directory' };
  }

  const entries = node.children
    .map((child) => (child.type === 'directory' ? `${child.name}/` : child.name))
    .sort((a, b) => a.localeCompare(b));

  return { entries };
}

export function readFile(path: string[], targetPath: string): { content?: string; error?: string } {
  const normalized = normalizeSegments(targetPath, path);
  const node = findNode(normalized);

  if (!node) {
    return { error: `cat: ${targetPath}: No such file or directory` };
  }

  if (node.type !== 'file') {
    return { error: `cat: ${targetPath}: Is a directory` };
  }

  return { content: node.content };
}

export function formatPath(path: string[]): string {
  if (path.length === 0) {
    return '/';
  }

  return `/${path.join('/')}`;
}

export function getFileSystemRoot(): DirectoryNode {
  return fileSystem;
}
