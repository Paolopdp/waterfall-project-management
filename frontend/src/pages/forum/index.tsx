import { useState } from "react";
import { useQuery, useMutation, useQueryClient } from "react-query";
import { useTranslation } from "next-i18next";
import { serverSideTranslations } from "next-i18next/serverSideTranslations";
import type { GetServerSideProps } from "next";
import Head from "next/head";
import { forumService } from "../../services/forum";
import ThreadList from "../../components/forum/ThreadList";
import ThreadForm from "../../components/forum/ThreadForm";
import SearchBar from "../../components/forum/SearchBar";
import TagFilter from "../../components/forum/TagFilter";
import Card from "@/widgets/card";

interface ThreadSearchParams {
  query?: string;
  tags?: string[];
  authorId?: string;
  fromDate?: string;
  toDate?: string;
  limit?: number;
  offset?: number;
}

export default function ForumPage() {
  const { t } = useTranslation("common");
  const queryClient = useQueryClient();
  const [searchParams, setSearchParams] = useState<ThreadSearchParams>({
    limit: 10,
    offset: 0,
  });
  const [showNewThreadForm, setShowNewThreadForm] = useState(false);

  const { data: threads, isLoading } = useQuery(
    ["threads", searchParams],
    () => forumService.searchThreads(searchParams),
    {
      keepPreviousData: true,
    }
  );

  const { data: tags } = useQuery("forumTags", forumService.getTags);

  const createThread = useMutation(forumService.createThread, {
    onSuccess: () => {
      queryClient.invalidateQueries("threads");
      setShowNewThreadForm(false);
    },
  });

  const handleSearch = (newParams: Partial<ThreadSearchParams>) => {
    setSearchParams((prev) => ({ ...prev, ...newParams, offset: 0 }));
  };

  const handleLoadMore = () => {
    setSearchParams((prev) => ({
      ...prev,
      offset: (prev.offset || 0) + (prev.limit || 10),
    }));
  };

  return (
    <>
      <Head>
        <title>
          {t("forum.title")} | {t("common.appName")}
        </title>
      </Head>

      <main className="container mx-auto px-4 py-8">
        <div className="flex justify-between items-center mb-6">
          <h1 className="text-3xl font-bold">{t("forum.title")}</h1>
          <button
            onClick={() => setShowNewThreadForm(true)}
            className="btn btn-primary"
          >
            {t("forum.newThread")}
          </button>
        </div>

        <Card className="mb-6">
          <div className="space-y-4">
            <SearchBar onSearch={(query) => handleSearch({ query })} />
            <TagFilter
              tags={tags || []}
              onTagsChange={(tags) => handleSearch({ tags })}
            />
          </div>
        </Card>

        {showNewThreadForm && (
          <ThreadForm
            onSubmit={createThread.mutate}
            onCancel={() => setShowNewThreadForm(false)}
            tags={tags || []}
          />
        )}

        <ThreadList
          threads={threads || []}
          isLoading={isLoading}
          onLoadMore={handleLoadMore}
        />
      </main>
    </>
  );
}

export const getServerSideProps: GetServerSideProps = async ({ locale }) => {
  return {
    props: {
      ...(await serverSideTranslations(locale ?? "en", ["common"])),
    },
  };
};
