import useSearchGraphResult from '../hooks/useSearchGraphResult';
import type { SearchGraphQuery, SearchGraphResult } from '../../shared/types/search_graph';
import { SigmaContainer, useLoadGraph } from '@react-sigma/core';
import Graph from 'graphology';
import { forwardRef, useImperativeHandle, useEffect, useRef, FC } from "react";
import { useState, useLayoutEffect } from 'react';
import { useWorkerLayoutForceAtlas2 } from '@react-sigma/layout-forceatlas2';
import { hslToRgb, rgbToHex } from '@mui/system';
import "@react-sigma/core/lib/style.css";
import Box from '@mui/material/Box'; 
import { useTheme, useMediaQuery, Typography } from '@mui/material';

const Fa2 = forwardRef((_props, ref) => {
  const { start, kill, stop } = useWorkerLayoutForceAtlas2({ settings: { slowDown: 10 } });

  useImperativeHandle(ref, () => ({ start, stop }));
  useEffect(() => {
    start();

    return () => {
      kill();
    };
  }, [start, kill, stop]);

  return null;
});

const WikipediaGraph: FC<{ data: SearchGraphResult | null }> = ({ data }: {data: SearchGraphResult | null}) => {
  const loadGraph = useLoadGraph();

  useEffect(() => {
    if (!data) return;
    const graph = new Graph();

    data.nodes.forEach(node => {
      const gamma = 0.3;
      let t = Math.min(Math.max(node.distance / data.end_node.distance, 0), 1);
      t = Math.pow(t, gamma);
      const hue = (1 - t) * 240;
      const hsl = `hsl(${hue.toFixed(0)}, 80%, 70%)`;
      const rgb = hslToRgb(hsl);  

      let x = Math.random();
      let y = Math.random();
      let size = 10;

      graph.addNode(node.id, {
        label: node.title,
        size: size,
        color: rgbToHex(rgb),
        x: x,
        y: y,
      });
    });

    data.edges.forEach(edge => {
      graph.addEdge(edge[0], edge[1]);
    });

    loadGraph(graph);
  }, [loadGraph, data]);

  return null;
};

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
  const fa2Ref = useRef<{ start: () => void; kill: () => void; stop: () => void }>(null);
  const theme = useTheme();
  const isXs = useMediaQuery(theme.breakpoints.only('xs'));
  const isSm = useMediaQuery(theme.breakpoints.only('sm'));
  const isMd = useMediaQuery(theme.breakpoints.only('md'));
  const isLg = useMediaQuery(theme.breakpoints.only('lg'));
  const isXl = useMediaQuery(theme.breakpoints.only('xl'));
  const boxRef = useRef<HTMLDivElement>(null);
  const [boxWidth, setBoxWidth] = useState<number>(300);

  const sigmaStyle = () => {
    if (isXs) return { width: boxWidth, height: 300 };
    else if (isSm) return { width: 600, height: 300 };
    else if (isMd) return { width: 900, height: 400 };
    else if (isLg) return { width: 1200, height: 500 };
    else if (isXl) return { width: 1200, height: 500 };
  };

  useLayoutEffect(() => {
    if (isXs && boxRef.current) {
      setBoxWidth(boxRef.current.offsetWidth);
    }
  }, [isXs, data, loading]);

  useEffect(() => {
    if (fa2Ref.current) {
      fa2Ref.current.start();

      setTimeout(() => {
        fa2Ref.current?.stop();
      }, 3000);
    }
  }, [fa2Ref, data]);

  return (
    <>
      {loading && <p>Loading...</p>}
      {data && (
        <>
          <Box sx={{ width: '100%' }} ref={boxRef}>
            <Typography variant="h6" align="center">
              {data.start_node.title}から{data.end_node.title}へは{data.end_node.distance}リンクで到達できます。
            </Typography>
            <Typography variant="body2" align="center" color="text.secondary" sx={{ mt: 0.5 }}>
              発見ノード数: {data.discovered_nodes} 探索ノード数: {data.visited_nodes}
              {isXs ? <br />: ' '}
              処理時間: {(data.duration.secs * 1000 + data.duration.nanos / 1000.0 / 1000.0).toFixed(3) } ミリ秒
            </Typography>
          </Box>
          <Box sx={{ width: '100%', display: 'flex', justifyContent: 'center' }} ref={boxRef}>
            <SigmaContainer style={sigmaStyle()} settings={sigmaSettings}>
              <WikipediaGraph data={data} />
              <Fa2 ref={fa2Ref} />
            </SigmaContainer>
          </Box>
        </>
      )}
      {!query && <p>Please enter a start and end title to see the graph.</p>}
      {query && !loading && !data && <p>No data available</p>}
    </>
  )
}