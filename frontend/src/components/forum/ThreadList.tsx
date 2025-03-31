import { Thread } from '@/types/forum';
import { useTranslation } from 'next-i18next';
import Link from 'next/link';
import { formatDistance } from 'date-fns';

interface ThreadListProps {
  threads: Thread[];
  isLoading: boolean;
  onLoadMore: () => void;
}

export default function ThreadList({ threads, isLoading, onLoadMore }: ThreadListProps) {
  const { t } = useTranslation('common');

  const formatCreatedAt = (date: string | null | undefined) => {
    if (!date) return '';
    try {
      return formatDistance(new Date(date), new Date(), { addSuffix: true });
    } catch (error) {
      return '';
    }
  };

  if (isLoading) {
    return <div className="animate-pulse">{t('common.loading')}</div>;
  }

  return (
    <div className="space-y-4">
      {threads.map((thread) => (
        <div key={thread.id} className="border rounded-lg p-4 hover:bg-gray-50">
          <Link href={`/forum/${thread.id}`}>
            <div className="flex justify-between items-start">
              <div>
                <h3 className="text-lg font-semibold">{thread.title}</h3>
                <p className="text-gray-600 mt-1">{thread.content.substring(0, 150)}...</p>
                <div className="flex gap-2 mt-2">
                  {thread.tags?.map((tag) => (
                    <span
                      key={tag.id}
                      className="bg-blue-100 text-blue-800 text-xs px-2 py-1 rounded"
                    >
                      {tag.name}
                    </span>
                  ))}
                </div>
              </div>
              <div className="text-sm text-gray-500">
                {formatCreatedAt(thread.createdAt)}
              </div>
            </div>
          </Link>
        </div>
      ))}

      {threads.length > 0 && (
        <button
          onClick={onLoadMore}
          className="w-full py-2 text-center text-gray-600 hover:text-gray-800"
        >
          {t('forum.loadMore')}
        </button>
      )}

      {threads.length === 0 && (
        <div className="text-center py-8 text-gray-500">
          {t('forum.noThreads')}
        </div>
      )}
    </div>
  );
}
