import { useRouter } from 'next/router';
import { useQuery, useMutation, useQueryClient } from 'react-query';
import { useTranslation } from 'next-i18next';
import { serverSideTranslations } from 'next-i18next/serverSideTranslations';
import { useForm } from 'react-hook-form';
import { forumService } from '@/services/forum';
import { formatDistance } from 'date-fns';
import type { Thread, Reply } from '@/types/forum';

export default function ThreadView() {
  const router = useRouter();
  const { id } = router.query;
  const { t } = useTranslation('common');
  const queryClient = useQueryClient();
  const { register, handleSubmit, reset } = useForm();

  // Get thread from React Query cache if available
  const existingThread = queryClient.getQueryData<Thread>(['thread', id]);

  // Only fetch thread if not in cache
  const { data: thread, isLoading: threadLoading } = useQuery<Thread>(
    ['thread', id],
    () => forumService.getThread(id as string),
    {
      enabled: !!id && !existingThread,
      initialData: existingThread
    }
  );

  const { data: replies, isLoading: repliesLoading } = useQuery<Reply[]>(
    ['replies', id],
    () => forumService.getReplies(id as string),
    { enabled: !!id }
  );

  const createReply = useMutation(
    (data: { content: string }) =>
      forumService.createReply(id as string, data.content),
    {
      onSuccess: () => {
        queryClient.invalidateQueries(['replies', id]);
        reset();
      },
    }
  );

  const formatCreatedAt = (date: string | null | undefined) => {
    if (!date) return '';
    try {
      return formatDistance(new Date(date), new Date(), { addSuffix: true });
    } catch (error) {
      return '';
    }
  };

  if (!thread || repliesLoading) {
    return <div className="text-center py-4">{t('common.loading')}</div>;
  }

  return (
    <div className="container mx-auto px-4 py-6">
      <div className="mb-8">
        <h1 className="text-2xl font-bold mb-4">{thread.title}</h1>
        <div className="bg-white rounded-lg shadow p-6">
          <div className="prose max-w-none">{thread.content}</div>
          <div className="text-sm text-gray-500 mt-2">
            {formatCreatedAt(thread.createdAt)}
          </div>
        </div>
      </div>

      <div className="mb-8">
        <h2 className="text-xl font-bold mb-4">{t('forum.replies')}</h2>
        <div className="space-y-4">
          {replies?.map((reply) => (
            <div key={reply.id} className="bg-white rounded-lg shadow p-4">
              <div className="prose max-w-none">{reply.content}</div>
              <div className="text-sm text-gray-500 mt-2">
                {formatCreatedAt(reply.createdAt)}
              </div>
            </div>
          ))}
        </div>
      </div>

      <div className="bg-white rounded-lg shadow p-4">
        <form onSubmit={handleSubmit((data) => createReply.mutate(data))}>
          <textarea
            {...register('content', { required: true })}
            className="w-full p-2 border rounded"
            rows={4}
            placeholder={t('forum.writeReply')}
          />
          <button
            type="submit"
            className="mt-2 bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600"
            disabled={createReply.isLoading}
          >
            {createReply.isLoading ? t('common.sending') : t('forum.reply')}
          </button>
        </form>
      </div>
    </div>
  );
}

export async function getStaticProps({ locale }: { locale: string }) {
  return {
    props: {
      ...(await serverSideTranslations(locale, ['common'])),
    },
  };
}

export async function getStaticPaths() {
  return {
    paths: [],
    fallback: true,
  };
}
