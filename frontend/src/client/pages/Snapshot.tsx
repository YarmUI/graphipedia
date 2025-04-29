import RawGraph from '../components/RawGraph'
import type { SearchGraphQuery } from '../../shared/types/search_graph';
import { useSearchParams } from 'react-router-dom';
import CssBaseline from '@mui/material/CssBaseline';
import { useEffect, useState } from 'react';


const Snapshot = () => {
  const [searchParams, _] = useSearchParams();
  const [query, setQuery] = useState<SearchGraphQuery | null>(null);

  useEffect(() => {
    const url_start = searchParams.get('start') || '';
    const url_end = searchParams.get('end') || '';
    setQuery({ start: url_start, end: url_end });
  },[]);


  return (
    <>
      <CssBaseline />
      <RawGraph query={query}/>
    </>
  )
}

export default Snapshot;