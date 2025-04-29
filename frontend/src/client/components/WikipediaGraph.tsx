import Graph from 'graphology';
import { useLayoutForceAtlas2 } from '@react-sigma/layout-forceatlas2';
import { hslToRgb, rgbToHex } from '@mui/system';
import { useEffect, FC } from "react";
import { useLoadGraph } from '@react-sigma/core';
import type { SearchGraphResult } from '../../shared/types/search_graph';

const WikipediaGraph: FC<{ data: SearchGraphResult | null }> = ({ data }: {data: SearchGraphResult | null}) => {
  const loadGraph = useLoadGraph();
  const { positions, assign } = useLayoutForceAtlas2({ iterations: 300, settings: { scalingRatio: 0.5, adjustSizes: true } });

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

      let x = Math.random() + (node.distance + 1) * 5.0;
      let y = Math.random();
      let size = 10;

      if(node.id == data.start_node.id) {
        size = 20;
      }

      if (node.id == data.end_node.id) {
        size = 20;
      }

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
    assign();
  }, [loadGraph, data, assign, positions]);

  return null;
};

export default WikipediaGraph;