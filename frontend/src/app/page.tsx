import Graphipedia from "@/components/Graphipedia";
import fetchGraphSearch from "@/lib/fetchGraphSearch";

export default async function Home({ searchParams }: { searchParams: { start?: string, end?: string } }) {
  const { start, end } = await searchParams;
  const data = start && end ? await fetchGraphSearch({ start, end }) : undefined;

  return (
    <div>
      <Graphipedia initialData={data} />
    </div>
  );
}
