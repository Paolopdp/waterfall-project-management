import { useState } from 'react';
import { useQuery } from 'react-query';
import { useTranslation } from 'next-i18next';
import { serverSideTranslations } from 'next-i18next/serverSideTranslations';

interface Thread {
  id: string;
  title: string;
  author: string;
  createdAt: string;
  lastReply: string;
  replyCount: number;
}

export default function Forum() {
  const { t } = useTranslation('common');
  const [searchTerm, setSearchTerm] = useState('');

  const { data: threads, isLoading } = useQuery<Thread[]>('threads', async () => {
    const response = await fetch('/api/forum/threads');
    if (!response.ok) throw new Error('Network response was not ok');
    return response.json();
  });

  return (
    <div className="container mx-auto px-4">
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold">{t('forum.title')}</h1>
        <button
          className="bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600"
          onClick={() => {/* TODO: Implement new thread creation */}}
        >
          {t('forum.newThread')}
        </button>
      </div>

      <div className="mb-4">
        <input
          type="text"
          placeholder={t('forum.searchPlaceholder')}
          className="w-full px-4 py-2 border rounded"
          value={searchTerm}
          onChange={(e) => setSearchTerm(e.target.value)}
        />
      </div>

      {isLoading ? (
        <div className="text-center py-4">Loading...</div>
      ) : (
        <div className="bg-white shadow rounded-lg">
          <table className="min-w-full">
            <thead>
              <tr className="bg-gray-50">
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  {t('forum.thread')}
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  {t('forum.author')}
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  {t('forum.replies')}
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  {t('forum.lastReply')}
                </th>
              </tr>
            </thead>
            <tbody className="bg-white divide-y divide-gray-200">
              {threads?.map((thread) => (
                <tr key={thread.id} className="hover:bg-gray-50 cursor-pointer">
                  <td className="px-6 py-4">
                    <div className="text-sm font-medium text-gray-900">
                      {thread.title}
                    </div>
                  </td>
                  <td className="px-6 py-4 text-sm text-gray-500">
                    {thread.author}
                  </td>
                  <td className="px-6 py-4 text-sm text-gray-500">
                    {thread.replyCount}
                  </td>
                  <td className="px-6 py-4 text-sm text-gray-500">
                    {new Date(thread.lastReply).toLocaleDateString()}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
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