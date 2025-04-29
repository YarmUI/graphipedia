import useSearchGraphResult from '../hooks/useSearchGraphResult';
import type { SearchGraphQuery } from '../../shared/types/search_graph';
import { SigmaContainer } from '@react-sigma/core';
import { useEffect, useState } from "react";
import "@react-sigma/core/lib/style.css";
import Box from '@mui/material/Box'; 
import WikipediaGraph from './WikipediaGraph';

const sigmaSettings = {
  allowInvalidContainer: true,
  defaultEdgeType: 'arrow',
  labelDensity: 0.07,
  labelGridCellSize: 60,
  labelFont: 'Lato, sans-serif',
  zIndex: true,
};

export default ({query}: {query: SearchGraphQuery | null }) => {
  const { data, loading } = useSearchGraphResult(query);
  const [isRouteFound, setIsRouteFound] = useState<boolean>(false);

  useEffect(() => {
    if (data && data.route_found) {
      setIsRouteFound(true);
      return;
    }
    setIsRouteFound(false);
  }, [loading, data]);

  return (
    <>
      {data && isRouteFound && (
          <Box sx={{ width: '100%', display: 'flex', justifyContent: 'center' }}>
            <SigmaContainer style={{ width: 1200, height: 630 }} settings={sigmaSettings}>
              <WikipediaGraph data={data} />
            </SigmaContainer>
          </Box>
      )}
    </>
  )
}