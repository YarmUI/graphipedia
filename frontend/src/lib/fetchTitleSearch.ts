import { components } from '../../types/api-types';

const BASE_URL = process.env.API_BASE_URL || 'http://localhost:3001';

type TitleSearchResult = components['schemas']['TitleSearchResult'];
type TitleSearchQuery = components['schemas']['TitleSearchQuery'];

export default async function fetchTitleSearch(query: TitleSearchQuery): Promise<TitleSearchResult> {
  const url = new URL('/api/search', BASE_URL);
  url.searchParams.append('query', query.query);
  if (query.limit) {
    url.searchParams.append('limit', query.limit.toString());
  }
  const response = await fetch(url.toString(), {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
    },
    cache: 'force-cache'
  });

  if (!response.ok) {
    throw new Error(`Failed to fetch Title Search: ${response.statusText}`);
  }

  const data: TitleSearchResult = await response.json();
  return data;
}