'use client';

import { components } from '../../types/api-types';
import { useState, useEffect, use } from 'react';
import Button from '@mui/material/Button'
import TitleSearchField from "@/components/TitleSearchField";
import useSWR from 'swr';
import fetchGraphSearch from '@/lib/fetchGraphSearch';
import { useRouter } from 'next/navigation';

type GraphSearchResult = components['schemas']['GraphSearchResult'];
type GraphSearchQuery = components['schemas']['GraphSearchQuery'];
type TitleSearchResultItem = components['schemas']['TitleSearchResultItem'];

export default ({ initialData }: { initialData?: GraphSearchResult }) => {
  const router = useRouter();
  const initialQuery = initialData?.start_node_search_result?.title && initialData?.end_node_search_result?.title ?
    { start: initialData.start_node_search_result.title, end: initialData.end_node_search_result.title } : null;

  const [start, setStart] = useState<TitleSearchResultItem | null>(initialData?.start_node_search_result || null);
  const [end, setEnd] = useState<TitleSearchResultItem | null>(initialData?.end_node_search_result || null);
  const [query, setQuery] = useState<GraphSearchQuery | null>(initialQuery);
  const { data, error } = useSWR<GraphSearchResult>(
    query ? ['graphSearch', query] : null,
    () => fetchGraphSearch(query as GraphSearchQuery),
    { fallbackData: initialData }
  );

  const handleSearch = (_: any) => {
    if (!start || !end) {
      return;
    }

    const params = new URLSearchParams();
    params.set('start', start.title);
    params.set('end', end.title);

    router.replace(`?${params.toString()}`);
    setQuery({ start: start.title, end: end.title });
  };

  return (
    <div>
      <div>
        {JSON.stringify(data ? data : {}, null, 2)}
      </div>
      <div>
        <TitleSearchField label="スタート" value={start} setValue={setStart} />
        <TitleSearchField label="ゴール" value={end} setValue={setEnd} />
        <Button variant="contained" onClick={handleSearch}>最短経路を探索</Button>      
      </div>
    </div>
  );
};