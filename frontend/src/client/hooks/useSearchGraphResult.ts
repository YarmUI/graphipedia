import { useState, useEffect } from 'react'
import type { SearchGraphResult,  SearchGraphQuery } from '../../shared/types/search_graph'

const useSearchGraphResult = (query: SearchGraphQuery | null) => {

  const [data, setData]       = useState<SearchGraphResult | null>(null);
  const [loading, setLoading] = useState<boolean>(false);
  const [error, setError]     = useState<string | null>(null)

  useEffect(() => {
    if (!query) {
      setData(null);
      return;
    }
    setLoading(true);

    const params = new URLSearchParams({
      start: query.start,
      end: query.end,
    });

    fetch(`/api/graph_search?${params.toString()}`)
      .then((res) => {
        if (!res.ok) throw new Error(`Status ${res.status}`)
        return res.json() as Promise<SearchGraphResult>
      })
      .then((json) => setData(json))
      .catch((e: any) => setError(e.message))
      .finally(() => setLoading(false))

  }, [query])

  return { data, loading, error }
};

export default useSearchGraphResult;