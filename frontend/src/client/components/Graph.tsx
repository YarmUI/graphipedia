import useSearchGraphResult from '../hooks/useSearchGraphResult';
import type { SearchGraphQuery } from '../../shared/types/search_graph';

export default ({query}: {query: SearchGraphQuery | null }) => {
  const { data, loading } = useSearchGraphResult(query);

  return (
    <div>
      {loading && <p>Loading...</p>}
      {data && (
        <div>
          <h2>Graph Data</h2>
          <pre>{JSON.stringify(data, null, 2)}</pre>
        </div>
      )}
      {!query && <p>Please enter a start and end title to see the graph.</p>}
      {query && !loading && !data && <p>No data available</p>}
    </div>
  )
}