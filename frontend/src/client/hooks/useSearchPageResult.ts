import { useState, useEffect } from 'react'
import type { SearchPageResult,  SearchPageQuery } from '../../shared/types/search_page'

const useSearchPageResult = (query: SearchPageQuery) => {

  const [data, setData]       = useState<SearchPageResult | null>(null);
  const [loading, setLoading] = useState<boolean>(false);
  const [error, setError]     = useState<string | null>(null)

  useEffect(() => {
    if (!query.query || query.query.length === 0) {
      setData(null);
      return;
    }

    setLoading(true);

    const params = new URLSearchParams({
      query: query.query.toLowerCase(),
      limit: query.limit ? query.limit.toString() : 'null',
    });

    fetch(`/api/search?${params.toString()}`)
      .then((res) => {
        if (!res.ok) throw new Error(`Status ${res.status}`)
        return res.json() as Promise<SearchPageResult>
      })
      .then((json) => setData(json))
      .catch((e: any) => setError(e.message))
      .finally(() => setLoading(false))

  }, [query.query, query.limit])

  return { data, loading, error }
};

export default useSearchPageResult;