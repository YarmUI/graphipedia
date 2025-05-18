import { components } from '../../types/api-types';

const BASE_URL = process.env.API_BASE_URL || 'http://localhost:3001';

type GraphSearchResult = components['schemas']['GraphSearchResult'];
type GraphSearchQuery = components['schemas']['GraphSearchQuery'];

export default async function fetchGraphSearch(query: GraphSearchQuery): Promise<GraphSearchResult> {
  const url = new URL('/api/graph_search', BASE_URL);
  url.searchParams.append('start', query.start);
  url.searchParams.append('end', query.end);

  if (query.enable_date_related) {
    url.searchParams.append('enable_date_related', query.enable_date_related.toString());
  }
  if (query.enable_list_article) {
    url.searchParams.append('enable_list_article', query.enable_list_article.toString());
  }

  const response = await fetch(url.toString(), {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
    },
    cache: 'force-cache'
  });

  if (!response.ok) {
    throw new Error(`Failed to fetch Graph Search: ${response.statusText}`);
  }

  const data: GraphSearchResult = await response.json();
  return data;  
}
