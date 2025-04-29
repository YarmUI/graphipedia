import useSearchGraphResult from '../hooks/useSearchGraphResult';
import type { SearchGraphQuery } from '../../shared/types/search_graph';
import { SigmaContainer } from '@react-sigma/core';
import { useEffect, useRef } from "react";
import { useState, useLayoutEffect } from 'react';
import "@react-sigma/core/lib/style.css";
import Box from '@mui/material/Box'; 
import { useTheme, useMediaQuery, Typography } from '@mui/material';
import WikipediaGraph from './WikipediaGraph';
import {
  TwitterShareButton,
  TwitterIcon,
  LineShareButton,
  LineIcon,
  HatenaShareButton,
  HatenaIcon
} from 'react-share';

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
  const theme = useTheme();
  const isXs = useMediaQuery(theme.breakpoints.only('xs'));
  const isSm = useMediaQuery(theme.breakpoints.only('sm'));
  const isMd = useMediaQuery(theme.breakpoints.only('md'));
  const isLg = useMediaQuery(theme.breakpoints.only('lg'));
  const isXl = useMediaQuery(theme.breakpoints.only('xl'));
  const boxRef = useRef<HTMLDivElement>(null);
  const [boxWidth, setBoxWidth] = useState<number>(300);

  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [errMsg, setErrMsg] = useState<string | null>(null);
  const [isRouteFound, setIsRouteFound] = useState<boolean>(false);

  useEffect(() => {
    if (loading) {
      setIsLoading(true);
      setErrMsg(null);
      setIsRouteFound(false);
      return;
    }

    if (data && data.route_found) {
      setIsLoading(false);
      setErrMsg(null);
      setIsRouteFound(true);
      return;
    }

    if (data && data.start_not_found) {
      setErrMsg(`${query?.start}が見つかりませんでした。`);
      setIsLoading(false);
      setIsRouteFound(false);
      return;
    }

    if (data && data.end_not_found) {
      setErrMsg(`${query?.end}が見つかりませんでした。`);
      setIsLoading(false);
      setIsRouteFound(false);
      return;
    }

    if (data && !data.route_found) {
      setErrMsg('ルートが見つかりませんでした。');
      setIsLoading(false);
      setIsRouteFound(false);
      return;
    }

  }, [loading, data]);

  const graphWidth = () => {
    if (isXs) return boxWidth;
    else if (isSm) return 600;
    else if (isMd) return 900;
    else if (isLg) return 1200;
    else if (isXl) return 1200;
  }

  const graphHeight = () => {
    if (isXs) return 330;
    else if (isSm) return 480;
    else if (isMd) return 480;
    else if (isLg) return 480;
    else if (isXl) return 480;
  }

  const sigmaStyle = () => {
    return { width: graphWidth(), height: graphHeight() }
  };

  useLayoutEffect(() => {
    if (isXs && boxRef.current) {
      setBoxWidth(boxRef.current.offsetWidth);
    }
  }, [isXs, data, loading]);

  return (
    <>
      {!isLoading && !isRouteFound && !errMsg && (
        <Box
          sx={{
            width: '100%',
            height: "400px",
            display: 'flex',
            flexDirection: 'column',
            justifyContent: 'center',
            alignItems: 'center',
          }}
          ref={boxRef}
        >
          <Typography variant="h6" align="center">
            スタートの記事からゴールの記事までのWikipediaのリンクを探索します。
          </Typography>
        </Box>
      )}
      {isLoading && <p>Loading...</p>}
      {data && isRouteFound && (
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
            </SigmaContainer>
          </Box>
          <Box sx={{ width: '100%', display: 'flex', justifyContent: 'center', py: 1 }}>
            <Box sx={{ width: graphWidth(), display: 'flex', justifyContent: 'flex-end', gap: 1 }}>
              <TwitterShareButton
                url={window.location.href}
                title={`${data.start_node.title}から${data.end_node.title}へは${data.end_node.distance}リンクで到達できます。`}
                hashtags={['Graphipedia']}
              >
                <TwitterIcon size={32} round />
              </TwitterShareButton>
              <LineShareButton
                url={window.location.href}
                title={`${data.start_node.title}から${data.end_node.title}へは${data.end_node.distance}リンクで到達できます。`}
              >
                <LineIcon size={32} round />
              </LineShareButton>
              <HatenaShareButton
                url={window.location.href}
                title={`${data.start_node.title}から${data.end_node.title}へは${data.end_node.distance}リンクで到達できます。`}
              >
                <HatenaIcon size={32} round />
              </HatenaShareButton>
            </Box>
          </Box>
        </>
      )}
      {errMsg && (
        <Box sx={{ width: '100%' }} ref={boxRef}>
          <Typography variant="h6" align="center" color="error">
            {errMsg}
          </Typography>
        </Box>
      )}
    </>
  )
}