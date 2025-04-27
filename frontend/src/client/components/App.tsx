import Box from '@mui/material/Box'
import Container from '@mui/material/Container'
import Stack from '@mui/material/Stack'
import Button from '@mui/material/Button'
import TitleSearchField from "./TitleSearchField"
import Graph from "./Graph"
import type { SearchGraphQuery } from '../../shared/types/search_graph';
import { useState } from 'react';

function App() {

  const [query, setQuery] = useState<SearchGraphQuery | null>(null);
  const [start, setStart] = useState('');
  const [end, setEnd] = useState('');

  const handleSearch = () => {
    if (start && end) {
      setQuery({ start, end });
    }
  };

  const handleSwap = () => {
    if (start && end) {
      const _start = start;
      const _end = end;
      setStart(_end);
      setEnd(_start);
      setQuery({ start: _end, end: _start });
    }
  };

  return (
    <>
      <Container maxWidth="xl">
        <Stack
          justifyContent="center"
          alignItems="center"
          sx={{ width: '100%' }}
        >
          <Graph query={query} />
        </Stack>
        <Stack
          direction={{ xs: 'column', md: 'row' }}
          spacing={2}
          justifyContent="center"
          alignItems="center"
          sx={{ width: '100%' }}
        >
          <Box sx={{ width: '100%', maxWidth: 400 }}>
            <TitleSearchField label={"スタート"} value={start} setValue={setStart} />
          </Box>
          <Box sx={{ width: '100%', maxWidth: 400 }}>
            <TitleSearchField label={"ゴール"} value={end} setValue={setEnd} />
          </Box>
        </Stack>
        <Stack
          direction={{ xs: 'row' }}
          spacing={1}
          justifyContent="center"
          alignItems="center"
          sx={{ width: '100%', my: 2 }}
        >
          <Button variant="contained" onClick={handleSearch}>最短経路を探索</Button>
          <Button variant="contained" onClick={handleSwap}>入れ替えて探索</Button>
        </Stack>
      </Container>
    </>
  )
}

export default App
