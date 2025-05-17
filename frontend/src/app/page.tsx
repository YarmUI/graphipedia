//import fetchTitleSearch from "@/lib/fetchTitleSearch";
import TitleSearchField from "@/components/TitleSearchField";

export default async function Home({ searchParams }: { searchParams: { input?: string } }) {
  //const res = await fetchTitleSearch({ query: "Dog" });

  return (
    <div>
      <TitleSearchField label="Title Search Test" />
    </div>
  );
}
