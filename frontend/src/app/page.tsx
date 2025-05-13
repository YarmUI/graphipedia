import ClientComponent from "./ClientComponent";

export async function generateMetadata({ searchParams }: { searchParams: { [key: string]: string } }) {
  const input = searchParams.input || "";
  return {
    title: `Page - ${input}`,
    description: `This page is about ${input}.`,
  };
}

export default async function Home({ searchParams }: { searchParams: { input?: string } }) {
  const initValue = searchParams.input || "";
  const apiURL = "api/search";

  return (
    <ClientComponent initialValue={initValue} />
  );
}
