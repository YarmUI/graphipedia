import React from 'react';
import AppBar from '@mui/material/AppBar';
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import Container from '@mui/material/Container';
import CssBaseline from '@mui/material/CssBaseline';

const Layout: React.FC<{ children: React.ReactNode }> = ({ children }) => (
  <>
    <CssBaseline />
    <AppBar position="static">
      <Toolbar  sx={{ px: 3, minHeight: 40 }}>
        <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
          Graphipedia
        </Typography>
      </Toolbar>
    </AppBar>
    <Container maxWidth="xl" sx={{ minHeight: '80vh', py: 2 }}>
      {children}
    </Container>
  </>
);

export default Layout;