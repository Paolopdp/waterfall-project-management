import Link from "next/link";
import Head from "next/head";

export default function Home() {
  return (
    <>
      <Head>
        <title>Waterfall Resource Manager</title>
        <meta
          name="description"
          content="Resource management for waterfall projects"
        />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <main className="min-h-screen flex flex-col items-center justify-center p-4">
        <h1 className="text-4xl font-bold mb-6">Waterfall Resource Manager</h1>
        <p className="text-xl mb-8">Gestione risorse per progetti waterfall</p>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <Link
            href="/dashboard"
            className="p-6 border rounded-lg hover:bg-gray-50"
          >
            <h2 className="text-2xl font-bold">Dashboard</h2>
            <p>Panoramica dei progetti attivi</p>
          </Link>
          <Link
            href="/projects"
            className="p-6 border rounded-lg hover:bg-gray-50"
          >
            <h2 className="text-2xl font-bold">Progetti</h2>
            <p>Gestisci i tuoi progetti</p>
          </Link>
          <Link
            href="/resources"
            className="p-6 border rounded-lg hover:bg-gray-50"
          >
            <h2 className="text-2xl font-bold">Risorse</h2>
            <p>Gestisci le risorse disponibili</p>
          </Link>
          <Link
            href="/reports"
            className="p-6 border rounded-lg hover:bg-gray-50"
          >
            <h2 className="text-2xl font-bold">Report</h2>
            <p>Visualizza statistiche e report</p>
          </Link>
        </div>
      </main>
    </>
  );
}
