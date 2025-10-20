
import React from 'react';
import { Box, Text } from 'ink';

const Screen = ({ children }) => (
  <Box
    borderStyle="single"
    borderColor="green"
    flexDirection="column"
    padding={1}
    width="100%"
  >
    {children}
  </Box>
);

export default Screen;
