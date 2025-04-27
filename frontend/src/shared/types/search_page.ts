export interface SearchPageQuery {
  query: string;
  limit: null | number;
}

export interface SearchPageResult {
  search_query: SearchPageQuery;
  items: SearchPageResultItem[];
  duration: {
    secs: number;
    nanos: number;
  };
}

export interface SearchPageResultItem {
  title: string;
  id: number;
  is_redirect: boolean;
  redirected_title?: string;
  redirected_id?: number;
  link_count: number;
}